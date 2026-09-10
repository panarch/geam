use super::{Message, ReferenceId};
use futures_channel::oneshot;
use std::collections::BTreeMap;

/// Stable positions let suspended receives resume without rescanning old mail.
#[derive(Default)]
pub(crate) struct Mailbox {
    messages: BTreeMap<u64, Message>,
    next: u64,
    waiters: Vec<oneshot::Sender<()>>,
}

#[derive(Clone, Copy)]
pub(crate) struct Scan {
    next: u64,
    end: u64,
}

impl Mailbox {
    pub(crate) fn push(&mut self, message: Message) {
        self.messages.insert(self.next, message);
        self.next += 1;
        for waiter in self.waiters.drain(..) {
            let _ = waiter.send(());
        }
    }

    pub(crate) fn scan(&self, after: u64) -> Scan {
        Scan {
            next: after,
            end: self.next,
        }
    }

    pub(crate) fn next(&self, scan: &mut Scan) -> Option<(u64, Message)> {
        let (&position, message) = self.messages.range(scan.next..scan.end).next()?;
        scan.next = position + 1;
        Some((position, message.clone()))
    }

    pub(crate) fn remove(&mut self, position: u64) {
        self.messages.remove(&position);
    }

    pub(crate) fn wait(&mut self, scan: Scan) -> (oneshot::Receiver<()>, u64) {
        let (wake, waiter) = oneshot::channel();
        if self.next > scan.end {
            let _ = wake.send(());
        } else {
            self.waiters.retain(|waiter| !waiter.is_canceled());
            self.waiters.push(wake);
        }
        (waiter, scan.end)
    }

    pub(crate) fn demonitor(&mut self, monitor: ReferenceId) {
        self.messages.retain(
            |_, message| !matches!(message, Message::Down { monitor: id, .. } if *id == monitor),
        );
    }

    pub(crate) fn clear(&mut self) {
        self.messages.clear();
    }

    pub(crate) fn close(&mut self) {
        self.waiters.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::Mailbox;
    use crate::execution::{Message, Reason, ReferenceId};
    use geam_core::provider::advanced::NativeValue;

    #[test]
    fn overlapping_native_callback_waits_wake_without_cancelling_each_other() {
        let mut mailbox = Mailbox::default();
        let (mut first, _) = mailbox.wait(mailbox.scan(0));
        let (abandoned, _) = mailbox.wait(mailbox.scan(0));
        drop(abandoned);
        let (mut second, _) = mailbox.wait(mailbox.scan(0));
        assert_eq!(mailbox.waiters.len(), 2);
        assert_eq!(first.try_recv(), Ok(None));
        assert_eq!(second.try_recv(), Ok(None));
        mailbox.push(Message::Source(NativeValue::symbol("message")));
        assert_eq!(first.try_recv(), Ok(Some(())));
        assert_eq!(second.try_recv(), Ok(Some(())));
        let mut first_scan = mailbox.scan(0);
        let mut second_scan = mailbox.scan(0);
        let (position, _) = mailbox.next(&mut first_scan).unwrap();
        mailbox.remove(position);
        assert!(mailbox.next(&mut second_scan).is_none());
        let (mut first, _) = mailbox.wait(first_scan);
        let (mut second, _) = mailbox.wait(second_scan);
        mailbox.close();
        assert!(first.try_recv().is_err());
        assert!(second.try_recv().is_err());
    }

    #[test]
    fn scans_keep_order_and_a_fixed_end_across_arrival_removal_and_waits() {
        let mut mailbox = Mailbox::default();
        mailbox.push(Message::Source(NativeValue::symbol("first")));
        mailbox.push(Message::Source(NativeValue::symbol("second")));
        let mut scan = mailbox.scan(0);
        let (first, _) = mailbox.next(&mut scan).unwrap();
        assert_eq!(first, 0);
        mailbox.remove(first);
        mailbox.push(Message::Source(NativeValue::symbol("later")));
        let (second, _) = mailbox.next(&mut scan).unwrap();
        assert_eq!(second, 1);
        assert!(mailbox.next(&mut scan).is_none());
        let (mut arrived, after) = mailbox.wait(scan);
        assert_eq!(arrived.try_recv(), Ok(Some(())));
        let mut scan = mailbox.scan(after);
        let (later, _) = mailbox.next(&mut scan).unwrap();
        assert_eq!(later, 2);
        assert!(mailbox.next(&mut scan).is_none());
        let (mut waiting, _) = mailbox.wait(scan);
        assert_eq!(waiting.try_recv(), Ok(None));
        mailbox.push(Message::Source(NativeValue::symbol("wake")));
        assert_eq!(waiting.try_recv(), Ok(Some(())));
        assert_eq!(
            mailbox.messages.keys().copied().collect::<Vec<_>>(),
            [1, 2, 3]
        );
        mailbox.clear();
        assert!(mailbox.messages.is_empty());
        mailbox.push(Message::Source(NativeValue::symbol("after_flush")));
        assert_eq!(mailbox.messages.keys().copied().collect::<Vec<_>>(), [4]);
    }

    #[test]
    fn demonitor_removes_only_its_down_and_closure_or_dropped_waits_are_safe() {
        let mut mailbox = Mailbox::default();
        let first = ReferenceId::new();
        let second = ReferenceId::new();
        mailbox.push(Message::Source(NativeValue::symbol("ordinary")));
        for monitor in [first, second] {
            mailbox.push(Message::Down {
                monitor,
                reference: NativeValue::symbol("reference-view"),
                pid: NativeValue::symbol("pid-view"),
                reason: Reason::Normal,
            });
        }
        mailbox.demonitor(first);
        assert_eq!(mailbox.messages.keys().copied().collect::<Vec<_>>(), [0, 2]);
        let (mut waiter, _) = mailbox.wait(mailbox.scan(0));
        mailbox.close();
        assert_eq!(waiter.try_recv(), Err(futures_channel::oneshot::Canceled));
        let (waiter, _) = mailbox.wait(mailbox.scan(0));
        drop(waiter);
        mailbox.push(Message::Source(NativeValue::symbol("unobserved")));
        assert!(mailbox.waiters.is_empty());
    }
}
