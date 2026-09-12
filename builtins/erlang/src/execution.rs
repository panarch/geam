mod mailbox;

pub(crate) use mailbox::{Mailbox, Scan};

use ecow::EcoString;
use geam_core::execution::{
    ExecutionClock, ExecutionUnit, ExecutionUnitId, HostExecutionState, UnitExit,
};
use geam_core::provider::advanced::NativeValue;
use geam_core::{ExecutionError, HostFailure};
use priority_queue::PriorityQueue;
use std::borrow::Borrow;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::task::{Context, Poll};
use std::time::Instant;

/// Domain-owned process routing and lifecycle for `gleam_erlang`.
/// Retained Pids, Subjects and selectors do not own this state.
#[derive(Default)]
pub struct ErlangExecution {
    processes: BTreeMap<ExecutionUnitId, Process>,
    names: Names,
    atoms: BTreeSet<EcoString>,
    timers: PriorityQueue<Timer, Reverse<(Instant, ReferenceId)>>,
    sleeping: Option<(Instant, Sleep)>,
    terminated: VecDeque<Terminated>,
}

static NEXT_REFERENCE: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReferenceId(u64);

impl ReferenceId {
    pub(crate) fn new() -> Self {
        Self(NEXT_REFERENCE.fetch_add(1, Ordering::Relaxed))
    }

    pub(crate) fn number(self) -> u64 {
        self.0
    }
}

#[derive(Clone)]
pub(crate) enum Reason {
    Normal,
    Killed,
    Native(NativeValue),
    Failure(ExecutionError),
}

#[derive(Clone)]
pub(crate) enum Message {
    Source(NativeValue),
    Down {
        monitor: ReferenceId,
        reference: NativeValue,
        pid: NativeValue,
        reason: Reason,
    },
    Exit {
        pid: NativeValue,
        reason: Reason,
    },
}

struct Monitor {
    owner: ExecutionUnitId,
    reference: NativeValue,
    pid: NativeValue,
}

struct Link {
    /// The sender's typed identity is retained, never its mailbox or domain.
    pid: NativeValue,
}

struct Process {
    unit: ExecutionUnit,
    mailbox: Mailbox,
    trapping: bool,
    links: BTreeMap<ExecutionUnitId, Link>,
    watched_by: BTreeMap<ReferenceId, Monitor>,
    watching: BTreeMap<ReferenceId, ExecutionUnitId>,
    timers: BTreeSet<ReferenceId>,
}

#[derive(Default)]
struct Names {
    by_name: BTreeMap<EcoString, ExecutionUnit>,
    by_pid: BTreeMap<ExecutionUnitId, EcoString>,
}

struct Terminated {
    pid: ExecutionUnitId,
    process: Process,
    reason: Reason,
}

enum Cleanup {
    Timer(ReferenceId),
    Watching(ReferenceId, ExecutionUnitId),
    WatchedBy(ReferenceId, Monitor),
    Link(ExecutionUnitId, Link),
}

pub(crate) enum Destination {
    Pid(ExecutionUnitId),
    Name(EcoString),
}

struct Timer {
    id: ReferenceId,
    destination: Destination,
    message: NativeValue,
}

type Sleep = Pin<Box<dyn Future<Output = ()> + Send>>;

impl ErlangExecution {
    pub(crate) fn intern(&mut self, name: EcoString) -> Result<EcoString, HostFailure> {
        if name.chars().count() > 255 {
            return Err(HostFailure::new("atom name exceeds 255 Unicode codepoints"));
        }
        if let Some(name) = self.atoms.get(&name) {
            return Ok(name.clone());
        }
        if self.atoms.len() >= 1_048_576 {
            return Err(HostFailure::new("execution domain atom table is full"));
        }
        self.atoms.insert(name.clone());
        Ok(name)
    }

    pub(crate) fn existing_atom(&self, name: &str) -> Option<EcoString> {
        self.atoms.get(name).cloned()
    }

    pub(crate) fn alive(&self, pid: ExecutionUnitId) -> bool {
        self.processes
            .get(&pid)
            .is_some_and(|process| process.unit.is_active())
    }

    pub(crate) fn named(&self, name: &str) -> Option<ExecutionUnit> {
        self.names
            .by_name
            .get(name)
            .filter(|unit| unit.is_active())
            .cloned()
    }

    pub(crate) fn register(&mut self, pid: ExecutionUnitId, name: EcoString) -> bool {
        if name == "undefined" || self.named(&name).is_some() {
            return false;
        }
        self.unregister(&name);
        let Some(process) = self.processes.get(&pid) else {
            return false;
        };
        if !process.unit.is_active() || self.names.by_pid.contains_key(&pid) {
            return false;
        }
        self.names.insert(name, process.unit.clone());
        true
    }

    pub(crate) fn unregister(&mut self, name: &str) -> bool {
        self.names
            .remove_name(name)
            .is_some_and(|unit| unit.is_active())
    }

    pub(crate) fn send(&mut self, pid: ExecutionUnitId, message: Message) {
        if let Some(process) = self.processes.get_mut(&pid).filter(|p| p.unit.is_active()) {
            process.mailbox.push(message);
        }
    }

    pub(crate) fn mailbox(&mut self, pid: ExecutionUnitId) -> Option<&mut Mailbox> {
        self.processes
            .get_mut(&pid)
            .map(|process| &mut process.mailbox)
    }

    pub(crate) fn trap_exits(&mut self, pid: ExecutionUnitId, trapping: bool) {
        if let Some(process) = self.processes.get_mut(&pid) {
            process.trapping = trapping;
        }
    }

    pub(crate) fn monitor(
        &mut self,
        owner: ExecutionUnitId,
        target: ExecutionUnitId,
        id: ReferenceId,
        reference: NativeValue,
        pid: NativeValue,
    ) {
        if let Some(process) = self
            .processes
            .get_mut(&target)
            .filter(|p| p.unit.is_active())
        {
            process.watched_by.insert(
                id,
                Monitor {
                    owner,
                    reference,
                    pid,
                },
            );
            if let Some(owner) = self.processes.get_mut(&owner) {
                owner.watching.insert(id, target);
            }
        } else {
            self.send(
                owner,
                Message::Down {
                    monitor: id,
                    reference,
                    pid,
                    reason: Reason::Native(NativeValue::symbol("noproc")),
                },
            );
        }
    }

    pub(crate) fn demonitor(&mut self, owner: ExecutionUnitId, monitor: ReferenceId) {
        if let Some(process) = self.processes.get_mut(&owner) {
            let target = process.watching.remove(&monitor);
            process.mailbox.demonitor(monitor);
            if let Some(target) = target.and_then(|target| self.processes.get_mut(&target)) {
                target.watched_by.remove(&monitor);
            }
        }
    }

    pub(crate) fn link(
        &mut self,
        from: ExecutionUnitId,
        to: ExecutionUnitId,
        from_value: NativeValue,
        to_value: NativeValue,
    ) -> bool {
        if from == to {
            return true;
        }
        let Some(target) = self.processes.get_mut(&to).filter(|p| p.unit.is_active()) else {
            self.signal(
                from,
                to_value,
                Reason::Native(NativeValue::symbol("noproc")),
                false,
            );
            return true;
        };
        target.links.insert(from, Link { pid: to_value });
        if let Some(source) = self.processes.get_mut(&from) {
            source.links.insert(to, Link { pid: from_value });
        }
        true
    }

    pub(crate) fn unlink(&mut self, from: ExecutionUnitId, to: ExecutionUnitId) {
        if let Some(source) = self.processes.get_mut(&from) {
            source.links.remove(&to);
        }
        if let Some(target) = self.processes.get_mut(&to) {
            target.links.remove(&from);
        }
    }

    pub(crate) fn signal(
        &mut self,
        to: ExecutionUnitId,
        from: NativeValue,
        reason: Reason,
        self_exit: bool,
    ) {
        let Some(process) = self.processes.get(&to) else {
            return;
        };
        if process.trapping {
            self.send(to, Message::Exit { pid: from, reason });
        } else if self_exit || !matches!(reason, Reason::Normal) {
            self.terminate(to, reason);
        }
    }

    pub(crate) fn terminate(&mut self, pid: ExecutionUnitId, reason: Reason) {
        if self
            .processes
            .get(&pid)
            .is_some_and(|process| process.unit.cancel())
        {
            self.finish_process(pid, reason);
        }
    }

    fn finish_process(&mut self, pid: ExecutionUnitId, reason: Reason) {
        let Some(mut process) = self.processes.remove(&pid) else {
            return;
        };
        self.names.remove_pid(pid);
        process.mailbox.close();
        self.terminated.push_back(Terminated {
            pid,
            process,
            reason,
        });
    }

    fn finish_next(&mut self) -> bool {
        let Some(mut terminated) = self.terminated.pop_front() else {
            return false;
        };
        let pid = terminated.pid;
        match terminated.next() {
            Some(Cleanup::Timer(id)) => {
                self.remove_timer(id);
            }
            Some(Cleanup::Watching(id, target)) => {
                if let Some(target) = self.processes.get_mut(&target) {
                    target.watched_by.remove(&id);
                }
            }
            Some(Cleanup::WatchedBy(id, monitor)) => {
                let watching = self
                    .processes
                    .get_mut(&monitor.owner)
                    .is_some_and(|owner| owner.watching.remove(&id) == Some(pid));
                if watching {
                    self.send(
                        monitor.owner,
                        Message::Down {
                            monitor: id,
                            reference: monitor.reference,
                            pid: monitor.pid,
                            reason: terminated.reason.clone(),
                        },
                    );
                }
            }
            Some(Cleanup::Link(target, link)) => {
                if let Some(target_process) = self.processes.get_mut(&target)
                    && target_process.links.remove(&pid).is_some()
                {
                    self.signal(target, link.pid, terminated.reason.clone(), false);
                }
            }
            None => return true,
        }
        self.terminated.push_front(terminated);
        true
    }

    fn remove_timer(&mut self, id: ReferenceId) -> Option<(Timer, Instant)> {
        let (timer, Reverse((deadline, _))) = self.timers.remove(&id)?;
        self.unlink_timer(&timer);
        Some((timer, deadline))
    }

    fn unlink_timer(&mut self, timer: &Timer) {
        if let Destination::Pid(pid) = timer.destination
            && let Some(process) = self.processes.get_mut(&pid)
        {
            process.timers.remove(&timer.id);
        }
    }

    pub(crate) fn schedule(
        &mut self,
        id: ReferenceId,
        deadline: Instant,
        destination: Destination,
        message: NativeValue,
    ) {
        if let Destination::Pid(pid) = destination {
            let Some(process) = self.processes.get_mut(&pid).filter(|p| p.unit.is_active()) else {
                return;
            };
            process.timers.insert(id);
        }
        self.timers.push(
            Timer {
                id,
                destination,
                message,
            },
            Reverse((deadline, id)),
        );
    }

    pub(crate) fn cancel_timer(&mut self, id: ReferenceId, now: Instant) -> Option<u128> {
        let (timer, deadline) = self.remove_timer(id)?;
        if let Destination::Pid(pid) = timer.destination
            && !self.alive(pid)
        {
            return None;
        }
        Some(deadline.saturating_duration_since(now).as_millis())
    }
}

impl HostExecutionState for ErlangExecution {
    fn initialize(&mut self, metadata: geam_core::execution::ExecutionMetadata<'_>) {
        self.atoms.extend(
            [
                "true",
                "false",
                "nil",
                "ok",
                "error",
                "normal",
                "killed",
                "kill",
                "noproc",
                "undefined",
                "nonode@nohost",
                "process",
                "port",
                "DOWN",
                "EXIT",
                "selector",
                "anything",
                "geam_execution_error",
            ]
            .into_iter()
            .map(EcoString::from),
        );
        self.atoms
            .extend(metadata.native_constructor_tags().cloned());
    }

    fn started(&mut self, unit: ExecutionUnit) {
        self.processes.insert(
            unit.id(),
            Process {
                unit,
                mailbox: Mailbox::default(),
                trapping: false,
                links: BTreeMap::new(),
                watched_by: BTreeMap::new(),
                watching: BTreeMap::new(),
                timers: BTreeSet::new(),
            },
        );
    }

    fn finished(&mut self, unit: ExecutionUnitId, exit: &UnitExit) {
        self.finish_process(
            unit,
            match exit {
                UnitExit::Completed => Reason::Normal,
                UnitExit::Cancelled => Reason::Killed,
                UnitExit::Failed(error) => Reason::Failure(error.clone()),
            },
        );
    }

    fn close(&mut self) {
        self.timers.clear();
        self.sleeping = None;
        self.processes.clear();
        self.names = Names::default();
        self.atoms.clear();
        self.terminated.clear();
    }

    fn poll(&mut self, cx: &mut Context<'_>, clock: ExecutionClock<'_>) -> Poll<()> {
        let progress = if self.finish_next() {
            Poll::Ready(())
        } else {
            Poll::Pending
        };
        let ready = self.timers.pop_if(|_, Reverse((deadline, _))| {
            if self
                .sleeping
                .as_ref()
                .is_none_or(|(waiting, _)| waiting != deadline)
            {
                self.sleeping = None;
            }
            let (_, sleeping) = self
                .sleeping
                .get_or_insert_with(|| (*deadline, clock.sleep_until(*deadline)));
            sleeping.as_mut().poll(cx).is_ready()
        });
        let Some((timer, _)) = ready else {
            if self.timers.is_empty() {
                self.sleeping = None;
            }
            return progress;
        };
        self.sleeping = None;
        self.unlink_timer(&timer);
        let target = match timer.destination {
            Destination::Pid(pid) => Some(pid),
            Destination::Name(name) => self.named(&name).map(|unit| unit.id()),
        };
        if let Some(target) = target {
            self.send(target, Message::Source(timer.message));
        }
        Poll::Ready(())
    }
}

impl Names {
    fn insert(&mut self, name: EcoString, unit: ExecutionUnit) {
        self.by_pid.insert(unit.id(), name.clone());
        self.by_name.insert(name, unit);
    }

    fn remove_name(&mut self, name: &str) -> Option<ExecutionUnit> {
        let unit = self.by_name.remove(name)?;
        self.by_pid.remove(&unit.id());
        Some(unit)
    }

    fn remove_pid(&mut self, pid: ExecutionUnitId) {
        if let Some(name) = self.by_pid.remove(&pid) {
            self.by_name.remove(&name);
        }
    }
}

// The queue indexes each timer by identity; delivery data does not change its key.
impl PartialEq for Timer {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Timer {}

impl std::hash::Hash for Timer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Borrow<ReferenceId> for Timer {
    fn borrow(&self) -> &ReferenceId {
        &self.id
    }
}

impl Terminated {
    fn next(&mut self) -> Option<Cleanup> {
        if let Some(id) = self.process.timers.pop_first() {
            return Some(Cleanup::Timer(id));
        }
        if let Some((id, target)) = self.process.watching.pop_first() {
            return Some(Cleanup::Watching(id, target));
        }
        if let Some((id, monitor)) = self.process.watched_by.pop_first() {
            return Some(Cleanup::WatchedBy(id, monitor));
        }
        self.process
            .links
            .pop_first()
            .map(|(target, link)| Cleanup::Link(target, link))
    }
}

#[cfg(test)]
mod tests {
    use super::{Destination, ErlangExecution, Message, Reason, ReferenceId, Timer};
    use crate::test_support::with_units;
    use geam_core::execution::{HostExecutionState, UnitExit};
    use geam_core::provider::advanced::NativeValue;
    use std::borrow::Borrow;
    use std::hash::{DefaultHasher, Hash, Hasher};
    use std::time::{Duration, Instant};

    #[test]
    fn atom_table_limits_names_and_capacity_without_invalidating_existing_atoms() {
        let mut state = ErlangExecution::default();
        let longest = "x".repeat(255);
        assert_eq!(state.intern(longest.clone().into()).unwrap(), longest);
        assert_eq!(
            state.intern("x".repeat(256).into()).unwrap_err().message(),
            "atom name exceeds 255 Unicode codepoints",
        );
        for index in 1..1_048_576 {
            state.intern(index.to_string().into()).unwrap();
        }
        assert_eq!(state.intern(longest.clone().into()).unwrap(), longest);
        assert_eq!(
            state.intern("overflow".into()).unwrap_err().message(),
            "execution domain atom table is full",
        );
        assert_eq!(state.existing_atom("1").as_deref(), Some("1"));
        assert_eq!(state.existing_atom("overflow"), None);
        state.close();
        assert!(state.atoms.is_empty());
    }

    #[test]
    fn names_follow_logical_liveness_before_terminal_cleanup() {
        with_units(3, |units| {
            let mut state = ErlangExecution::default();
            let first = units[0].id();
            let replacement = units[1].id();
            assert!(!state.register(first, "service".into()));
            for unit in units {
                state.started(unit.clone());
            }
            assert!(!state.register(first, "undefined".into()));
            assert!(state.register(first, "service".into()));
            assert!(!state.register(first, "second".into()));
            assert!(!state.register(replacement, "service".into()));
            assert_eq!(state.named("service").unwrap().id(), first);
            assert!(units[0].cancel());
            assert!(!state.alive(first));
            state.send(first, Message::Source(NativeValue::symbol("late")));
            let mailbox = state.mailbox(first).unwrap();
            assert!(mailbox.next(&mut mailbox.scan(0)).is_none());
            assert!(state.named("service").is_none());
            assert!(!state.register(first, "dead".into()));
            assert!(state.register(replacement, "service".into()));
            state.finished(first, &UnitExit::Cancelled);
            while state.finish_next() {}
            assert_eq!(state.named("service").unwrap().id(), replacement);
            assert!(state.unregister("service"));
            assert!(!state.unregister("service"));
            assert!(state.register(units[2].id(), "temporary".into()));
            units[2].cancel();
            assert!(!state.unregister("temporary"));
            assert!(!state.names.by_name.contains_key("temporary"));
            assert!(state.names.by_pid.is_empty());
        });
    }

    #[test]
    fn late_exit_signals_cannot_replace_an_already_committed_core_cancellation() {
        with_units(2, |units| {
            let mut state = ErlangExecution::default();
            for unit in units {
                state.started(unit.clone());
            }
            let owner = units[0].id();
            let target = units[1].id();
            let monitor = ReferenceId::new();
            state.monitor(
                owner,
                target,
                monitor,
                NativeValue::symbol("monitor"),
                NativeValue::symbol("pid"),
            );
            assert!(units[1].cancel());
            state.terminate(target, Reason::Native(NativeValue::symbol("too_late")));
            state.finished(target, &UnitExit::Cancelled);
            while state.finish_next() {}
            let mailbox = state.mailbox(owner).unwrap();
            let mut scan = mailbox.scan(0);
            assert!(matches!(mailbox.next(&mut scan).unwrap().1,
                Message::Down { monitor: actual, reason: Reason::Killed, .. } if actual == monitor));
            assert!(mailbox.next(&mut scan).is_none());
        });
    }

    #[test]
    fn monitor_cancellation_and_late_terminal_cleanup_deliver_once() {
        with_units(3, |units| {
            let mut state = ErlangExecution::default();
            for unit in units {
                state.started(unit.clone());
            }
            let owner = units[0].id();
            let target = units[1].id();
            let removed = ReferenceId::new();
            let watched = ReferenceId::new();
            let reference = NativeValue::symbol("reference_payload");
            let pid = NativeValue::symbol("pid_payload");
            for id in [removed, watched] {
                state.monitor(owner, target, id, reference.clone(), pid.clone());
            }
            state.demonitor(owner, removed);
            assert_eq!(state.processes[&target].watched_by.len(), 1);
            state.terminate(target, Reason::Normal);
            state.finished(target, &UnitExit::Cancelled);
            while state.finish_next() {}
            let mailbox = state.mailbox(owner).unwrap();
            let mut scan = mailbox.scan(0);
            let (_, message) = mailbox.next(&mut scan).unwrap();
            assert!(
                matches!(message, Message::Down { monitor, reason: Reason::Normal, .. } if monitor == watched)
            );
            assert!(mailbox.next(&mut scan).is_none());
            state.demonitor(owner, watched);
            assert!(state.mailbox(owner).unwrap().next(&mut scan).is_none());
            let late = ReferenceId::new();
            state.monitor(owner, target, late, reference.clone(), pid.clone());
            let mailbox = state.mailbox(owner).unwrap();
            let mut scan = mailbox.scan(0);
            let (_, message) = mailbox.next(&mut scan).unwrap();
            assert!(matches!(message, Message::Down {
                    monitor,
                    reference,
                    pid,
                    reason: Reason::Native(reason),
                } if monitor == late
                    && reference.as_symbol().as_deref() == Some("reference_payload")
                    && pid.as_symbol().as_deref() == Some("pid_payload")
                    && reason.as_symbol().as_deref() == Some("noproc")));
            state.terminate(owner, Reason::Killed);
            state.demonitor(owner, late);
            let target = units[2].id();
            state.monitor(owner, target, late, reference, pid);
            state.terminate(target, Reason::Normal);
            while state.finish_next() {}
            assert!(state.processes.is_empty());
        });
    }

    #[test]
    fn links_respect_trapping_unlink_and_missing_peers() {
        with_units(4, |units| {
            let mut state = ErlangExecution::default();
            for unit in units {
                state.started(unit.clone());
            }
            let source = units[0].id();
            let target = units[1].id();
            let from = NativeValue::symbol("from");
            let to = NativeValue::symbol("to");
            assert!(state.link(source, source, from.clone(), from.clone()));
            assert!(state.processes[&source].links.is_empty());
            state.link(source, target, from.clone(), to.clone());
            state.unlink(source, target);
            assert!(state.processes[&source].links.is_empty());
            assert!(state.processes[&target].links.is_empty());
            state.trap_exits(source, true);
            state.link(source, target, from.clone(), to.clone());
            state.terminate(target, Reason::Normal);
            while state.finish_next() {}
            let mailbox = state.mailbox(source).unwrap();
            let mut scan = mailbox.scan(0);
            assert!(matches!(
                mailbox.next(&mut scan).unwrap().1,
                Message::Exit {
                    reason,
                    ..
                } if std::mem::discriminant(&reason) == std::mem::discriminant(&Reason::Normal)
            ));
            assert!(state.alive(source));
            state.link(source, target, from.clone(), to.clone());
            let mailbox = state.mailbox(source).unwrap();
            assert!(matches!(mailbox.next(&mut mailbox.scan(1)).unwrap().1,
                Message::Exit {
                    reason: Reason::Native(reason),
                    ..
                } if reason.as_symbol().as_deref() == Some("noproc")));
            state.trap_exits(source, false);
            state.signal(source, to.clone(), Reason::Normal, false);
            assert!(state.alive(source));
            state.signal(source, to.clone(), Reason::Normal, true);
            assert!(!state.alive(source));
            state.signal(source, to.clone(), Reason::Killed, false);
            state.trap_exits(source, true);
            state.unlink(source, target);
            let other = units[2].id();
            let watching = ReferenceId::new();
            state.monitor(other, units[3].id(), watching, from.clone(), to.clone());
            state.terminate(other, Reason::Killed);
            while state.finish_next() {}
            assert!(state.processes[&units[3].id()].watched_by.is_empty());
            state.link(source, units[3].id(), from, to);
            state.signal(
                units[3].id(),
                NativeValue::symbol("source"),
                Reason::Native(NativeValue::symbol("abnormal")),
                false,
            );
            assert!(!state.alive(units[3].id()));
            state.close();
            assert!(state.terminated.is_empty());
            assert!(state.processes.is_empty());
        });
    }

    #[test]
    fn cleanup_tolerates_peers_that_have_already_terminated() {
        with_units(2, |units| {
            let mut state = ErlangExecution::default();
            for unit in units {
                state.started(unit.clone());
            }
            let first = units[0].id();
            let second = units[1].id();
            state.link(
                first,
                second,
                NativeValue::symbol("first_pid"),
                NativeValue::symbol("second_pid"),
            );
            state.monitor(
                first,
                second,
                ReferenceId::new(),
                NativeValue::symbol("reference_payload"),
                NativeValue::symbol("second_pid"),
            );
            state.terminate(first, Reason::Killed);
            state.terminate(second, Reason::Killed);
            assert_eq!(state.terminated.len(), 2);
            while state.finish_next() {}
            assert!(state.terminated.is_empty());
            assert!(state.processes.is_empty());
        });
    }

    #[test]
    fn timer_queue_keys_are_identity_based_and_cancel_without_delivery() {
        let first = ReferenceId::new();
        let second = ReferenceId::new();
        let timer = Timer {
            id: first,
            destination: Destination::Name("one".into()),
            message: NativeValue::symbol("first_payload"),
        };
        let same = Timer {
            id: first,
            destination: Destination::Name("two".into()),
            message: NativeValue::symbol("second_payload"),
        };
        let different = Timer {
            id: second,
            destination: Destination::Name("one".into()),
            message: NativeValue::symbol("first_payload"),
        };
        assert!(timer == same);
        assert!(timer != different);
        assert_eq!(<Timer as Borrow<ReferenceId>>::borrow(&timer), &first);
        let mut expected = DefaultHasher::new();
        first.hash(&mut expected);
        for item in [&timer, &same] {
            let mut actual = DefaultHasher::new();
            item.hash(&mut actual);
            assert_eq!(actual.finish(), expected.finish());
        }

        let now = Instant::now();
        let mut state = ErlangExecution::default();
        state.schedule(first, now, timer.destination, timer.message);
        state.schedule(second, now, different.destination, different.message);
        assert_eq!(state.timers.len(), 2);
        assert_eq!(state.timers.peek().unwrap().0.id, first);
        assert_eq!(state.cancel_timer(first, now), Some(0));
        assert_eq!(state.timers.len(), 1);
        assert_eq!(state.timers.peek().unwrap().0.id, second);
        assert_eq!(state.cancel_timer(second, now), Some(0));
        assert!(state.timers.is_empty());
    }

    #[test]
    fn timers_are_removed_on_cancel_or_target_death_but_names_remain_late_bound() {
        with_units(2, |units| {
            let mut state = ErlangExecution::default();
            let pid = units[0].id();
            state.started(units[0].clone());
            let now = Instant::now();
            let id = ReferenceId::new();
            let message = NativeValue::symbol("message");
            state.schedule(
                id,
                now + Duration::from_millis(10),
                Destination::Pid(pid),
                message.clone(),
            );
            assert_eq!(state.cancel_timer(id, now), Some(10));
            assert!(state.processes[&pid].timers.is_empty());
            assert_eq!(state.cancel_timer(id, now), None);
            state.schedule(id, now, Destination::Pid(pid), message.clone());
            units[0].cancel();
            assert_eq!(state.cancel_timer(id, now), None);
            state.schedule(id, now, Destination::Pid(pid), message.clone());
            assert!(state.timers.is_empty());
            state.schedule(id, now, Destination::Pid(units[1].id()), message.clone());
            assert!(state.timers.is_empty());
            state.schedule(id, now, Destination::Name("later".into()), message.clone());
            assert_eq!(
                state.cancel_timer(id, now + Duration::from_millis(1)),
                Some(0)
            );
            state.started(units[1].clone());
            state.schedule(id, now, Destination::Pid(units[1].id()), message);
            state.terminate(units[1].id(), Reason::Normal);
            assert!(state.finish_next());
            assert!(state.timers.is_empty());
            while state.finish_next() {}
        });
    }
}
