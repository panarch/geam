use crate::schema::{StartError, StartResult, Started as StartedType, StartedConstructor};
use crate::{Call, Component, One, OtpProfile};
use geam::execution::ExecutionUnit;
use geam::gleam_erlang::service::{Processes, with_current_process};
use geam::gleam_erlang::{Component as ErlangComponent, Pid, PidSchema};
use geam::gleam_stdlib::provider_support::{GleamError, GleamOk};
use geam::host::{
    HostCallError, HostCustom, HostExecutionContext, HostExecutionError, HostType, HostTypeIndex0,
    HostTypeListEnd,
};
use geam::provider::advanced::NativeValue;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// The callback's typed result, projected before leaving its original codec.
pub(super) struct Started {
    pub unit: ExecutionUnit,
    pub pid: NativeValue,
    pub data: NativeValue,
}

#[derive(Clone, Copy)]
pub(super) enum Restart {
    Permanent,
    Transient,
    Temporary,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Shutdown {
    Finite(Duration),
    Infinite,
}

pub(super) struct RestartBudget {
    intensity: usize,
    period: Duration,
    attempts: VecDeque<Instant>,
}

pub(super) fn decode<'call, Profile: OtpProfile, Data: HostType>(
    mut call: Call<'call, Profile, ()>,
    result: HostCustom<'call, StartResult<Data>>,
) -> Result<Started, NativeValue> {
    if let Some((started, ())) =
        call.custom_fields::<GleamOk<StartedType<Data>, StartError>>(result)
    {
        let (pid, (data, ())) =
            call.provider_remaining_custom_fields::<StartedConstructor<Data>>(started);
        Ok(Started {
            unit: call
                .external_payload_with::<ErlangComponent<Profile>, PidSchema, HostTypeListEnd>(pid)
                .clone(),
            pid: call.native_value::<Pid>(pid),
            data: call.native_value::<Data>(data),
        })
    } else {
        let (error, ()) = call
            .provider_remaining_custom_fields::<GleamError<StartedType<Data>, StartError>>(result);
        Err(call.native_value::<StartError>(error))
    }
}

impl Restart {
    pub fn from_properties(properties: &NativeValue) -> Result<Self, HostCallError> {
        match property(properties, "restart")
            .and_then(|value| value.as_symbol())
            .as_deref()
        {
            Some("permanent") => Ok(Self::Permanent),
            Some("transient") => Ok(Self::Transient),
            Some("temporary") => Ok(Self::Temporary),
            _ => {
                Err(geam::HostFailure::new("child specification requires a restart policy").into())
            }
        }
    }

    pub fn after_exit(self, reason: &NativeValue) -> bool {
        match self {
            Self::Permanent => true,
            Self::Temporary => false,
            Self::Transient => {
                !(matches!(reason.as_symbol().as_deref(), Some("normal" | "shutdown"))
                    || (reason.len() == Some(2)
                        && reason
                            .index(0)
                            .and_then(|value| value.as_symbol())
                            .as_deref()
                            == Some("shutdown")))
            }
        }
    }
}

impl Shutdown {
    pub fn from_properties(properties: &NativeValue) -> Result<Self, HostCallError> {
        let value = property(properties, "shutdown").ok_or_else(|| {
            geam::HostFailure::new("child specification requires a shutdown policy")
        })?;
        if value.as_symbol().as_deref() == Some("infinity") {
            return Ok(Self::Infinite);
        }
        let milliseconds = value
            .as_int()
            .and_then(|value| u64::try_from(value).ok())
            .ok_or_else(|| {
                geam::HostFailure::new("child shutdown timeout must be nonnegative milliseconds")
            })?;
        Ok(Self::Finite(Duration::from_millis(milliseconds)))
    }
}

impl RestartBudget {
    pub fn from_flags(flags: &NativeValue) -> Result<Self, HostCallError> {
        let intensity = property(flags, "intensity")
            .and_then(|value| value.as_int())
            .and_then(|value| usize::try_from(value).ok())
            .ok_or_else(|| geam::HostFailure::new("restart intensity must be nonnegative"))?;
        let period = property(flags, "period")
            .and_then(|value| value.as_int())
            .and_then(|value| u64::try_from(value).ok())
            .filter(|value| *value > 0)
            .ok_or_else(|| geam::HostFailure::new("restart period must be positive seconds"))?;
        Ok(Self {
            intensity,
            period: Duration::from_secs(period),
            attempts: VecDeque::new(),
        })
    }

    pub fn allow(&mut self, now: Instant) -> bool {
        while self
            .attempts
            .front()
            .is_some_and(|first| now.duration_since(*first) >= self.period)
        {
            self.attempts.pop_front();
        }
        if self.attempts.len() >= self.intensity {
            return false;
        }
        self.attempts.push_back(now);
        true
    }
}

pub(super) async fn stop<Profile: OtpProfile>(
    context: &HostExecutionContext<'_, Profile, Component<Profile>, One<Pid>>,
    target: &ExecutionUnit,
    shutdown: Shutdown,
) -> Result<(), HostExecutionError> {
    let stopping = target.clone();
    let deadline = with_current_process(context, move |mut process, constructions| {
        let deadline = shutdown_deadline(process.call().clock().now(), shutdown)?;
        process.send_exit(
            &stopping,
            NativeValue::symbol("shutdown"),
            constructions.at::<HostTypeIndex0>(),
        );
        Ok(deadline)
    })
    .await?;
    loop {
        let stopping = target.clone();
        let receive = with_current_process(context, move |mut process, _| {
            if !process.processes().is_alive(&stopping) {
                return Ok::<_, HostCallError>(None);
            }
            process
                .receive_record(NativeValue::symbol("EXIT"), 2, deadline)
                .map(Some)
        })
        .await?;
        let Some(receive) = receive else {
            return Ok(());
        };
        let notified = if deadline.is_some() {
            receive.wait(context).await?.is_some()
        } else {
            receive.wait_forever(context).await?;
            true
        };
        if !notified {
            let stopping = target.clone();
            context
                .with_call(move |mut call| {
                    Processes::new(&mut call).kill(&stopping);
                })
                .await?;
            return Ok(());
        }
    }
}

fn shutdown_deadline(
    now: Instant,
    shutdown: Shutdown,
) -> Result<Option<Instant>, geam::HostFailure> {
    match shutdown {
        Shutdown::Infinite => Ok(None),
        Shutdown::Finite(duration) => now
            .checked_add(duration)
            .map(Some)
            .ok_or_else(|| geam::HostFailure::new("shutdown timeout exceeds the host clock range")),
    }
}

pub(super) fn property(properties: &NativeValue, name: &str) -> Option<NativeValue> {
    let mut index = 0;
    while let Some(property) = properties.index(index) {
        if property.index(0).and_then(|tag| tag.as_symbol()).as_deref() == Some(name) {
            return property.index(1);
        }
        index += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{Restart, RestartBudget, Shutdown, property};
    use crate::test_support::{Profile, run_source};
    use crate::{Call, Component};
    use geam::host::{HostCallCompletion, HostCallError, HostProviderModule};
    use geam::provider::BigInt;
    use geam::provider::advanced::NativeValue;
    use geam::{ModuleSource, PackageSource, Value};
    use std::time::{Duration, Instant};

    #[test]
    fn shutdown_deadlines_preserve_the_host_clock_boundary() {
        let now = Instant::now();
        assert_eq!(
            super::shutdown_deadline(now, Shutdown::Infinite).unwrap(),
            None
        );
        assert_eq!(
            super::shutdown_deadline(now, Shutdown::Finite(Duration::ZERO)).unwrap(),
            Some(now)
        );
        let mut last = now;
        for bit in (0..64).rev() {
            if let Some(next) = last.checked_add(Duration::from_secs(1 << bit)) {
                last = next;
            }
        }
        assert_eq!(
            super::shutdown_deadline(last, Shutdown::Finite(Duration::from_secs(1)))
                .err()
                .unwrap()
                .to_string(),
            "shutdown timeout exceeds the host clock range"
        );
    }

    #[test]
    fn restart_policies_distinguish_normal_shutdown_and_abnormal_reasons() {
        let normal = NativeValue::symbol("normal");
        let shutdown = NativeValue::symbol("shutdown");
        let killed = NativeValue::symbol("killed");
        let detailed = NativeValue::tuple([shutdown.clone(), NativeValue::symbol("reason")]);
        let other = NativeValue::tuple([killed.clone(), NativeValue::symbol("reason")]);
        assert!(Restart::Permanent.after_exit(&normal));
        assert!(!Restart::Temporary.after_exit(&killed));
        assert!(!Restart::Transient.after_exit(&normal));
        assert!(!Restart::Transient.after_exit(&shutdown));
        assert!(!Restart::Transient.after_exit(&detailed));
        assert!(Restart::Transient.after_exit(&killed));
        assert!(Restart::Transient.after_exit(&other));

        for (name, expected) in [
            ("permanent", true),
            ("transient", true),
            ("temporary", false),
        ] {
            let properties = NativeValue::tuple([NativeValue::tuple([
                NativeValue::symbol("restart"),
                NativeValue::symbol(name),
            ])]);
            assert_eq!(
                Restart::from_properties(&properties)
                    .unwrap()
                    .after_exit(&killed),
                expected
            );
        }
        let invalid = NativeValue::tuple([NativeValue::tuple([
            NativeValue::symbol("restart"),
            NativeValue::symbol("other"),
        ])]);
        assert!(Restart::from_properties(&invalid).is_err());
        assert!(Restart::from_properties(&NativeValue::tuple([])).is_err());
        assert!(property(&NativeValue::tuple([]), "restart").is_none());
    }

    #[test]
    fn native_numeric_policies_validate_their_ranges_and_expire_the_restart_window() {
        let provider = HostProviderModule::<Profile>::new("application", "main")
            .unwrap()
            .with_scoped_function::<Component<Profile>, (BigInt, BigInt, BigInt), (), _>(
                "check", check,
            )
            .unwrap();
        assert_eq!(
            run_source(
                [PackageSource::new(
                    "application",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "main",
                        "main.gleam",
                        r#"
@external(erlang, "fixture", "check") fn check(zero: Int, positive: Int, negative: Int) -> Nil
pub fn main() { check(0, 2, -1) }
"#
                    )]
                )],
                [provider]
            )
            .unwrap(),
            Value::Nil
        );
    }

    fn check<'call>(
        call: Call<'call, Profile, ()>,
        zero: BigInt,
        positive: BigInt,
        negative: BigInt,
    ) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
        let zero = call.native_values().integer(zero);
        let positive = call.native_values().integer(positive);
        let negative = call.native_values().integer(negative);
        let finite = NativeValue::tuple([NativeValue::tuple([
            NativeValue::symbol("shutdown"),
            positive.clone(),
        ])]);
        assert_eq!(
            Shutdown::from_properties(&finite).unwrap(),
            Shutdown::Finite(Duration::from_millis(2))
        );
        let infinite = NativeValue::tuple([NativeValue::tuple([
            NativeValue::symbol("shutdown"),
            NativeValue::symbol("infinity"),
        ])]);
        assert_eq!(
            Shutdown::from_properties(&infinite).unwrap(),
            Shutdown::Infinite
        );
        assert!(Shutdown::from_properties(&NativeValue::tuple([])).is_err());
        let invalid = NativeValue::tuple([NativeValue::tuple([
            NativeValue::symbol("shutdown"),
            negative.clone(),
        ])]);
        assert!(Shutdown::from_properties(&invalid).is_err());
        let flags = NativeValue::tuple([
            NativeValue::tuple([NativeValue::symbol("intensity"), positive.clone()]),
            NativeValue::tuple([NativeValue::symbol("period"), positive.clone()]),
        ]);
        let mut budget = RestartBudget::from_flags(&flags).unwrap();
        let now = Instant::now();
        assert!(budget.allow(now));
        assert!(budget.allow(now + Duration::from_secs(1)));
        assert!(!budget.allow(now + Duration::from_millis(1999)));
        assert!(budget.allow(now + Duration::from_secs(2)));
        assert!(!budget.allow(now + Duration::from_secs(2)));
        assert!(budget.allow(now + Duration::from_secs(3)));
        let no_restarts = NativeValue::tuple([
            NativeValue::tuple([NativeValue::symbol("intensity"), zero.clone()]),
            NativeValue::tuple([NativeValue::symbol("period"), positive.clone()]),
        ]);
        assert!(!RestartBudget::from_flags(&no_restarts).unwrap().allow(now));
        for invalid in [
            NativeValue::tuple([]),
            NativeValue::tuple([NativeValue::tuple([
                NativeValue::symbol("intensity"),
                negative,
            ])]),
        ] {
            assert!(RestartBudget::from_flags(&invalid).is_err());
        }
        let bad_period = NativeValue::tuple([
            NativeValue::tuple([NativeValue::symbol("intensity"), positive]),
            NativeValue::tuple([NativeValue::symbol("period"), zero]),
        ]);
        assert!(RestartBudget::from_flags(&bad_period).is_err());
        Ok(call.return_value(()))
    }
}
