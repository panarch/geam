//! An ordinary provider selecting original source identities in the shared mailbox.

#[cfg(test)]
#[path = "../tests/support/profile.rs"]
mod test_profile;

#[geam::provider(package = "selective_receive_service_fixture", modules = [selection])]
pub struct Component;

#[geam::module(
    path = "selective_receive_service_fixture/native",
    crate_path = geam,
    profile = geam::gleam_erlang::GleamErlangHostProfile,
    component = crate::Component,
    stores = selection,
)]
mod selection {
    use geam::gleam_erlang::service::{self, ProcessCall};
    use geam::host::native::NativeValues;
    use geam::provider::advanced::{
        Equality, Hashing, Inspection, NativeKind, NativeValue, RetainedExternalPayload,
    };
    use geam::provider::{BigInt, Call, EcoString, HostResult, Restore, StringValue, Value};
    use std::cell::Cell;
    use std::time::Duration;

    #[geam::external(name = "Key", retained)]
    pub struct Key {
        value: NativeValue,
    }

    impl RetainedExternalPayload for Key {
        fn source_equal(&self, context: &Equality<'_>, other: &Self) -> bool {
            self.value.source_equal(context, &other.value)
        }

        fn source_hash(&self, context: &Hashing<'_>) -> u64 {
            self.value.source_hash(context)
        }

        fn inspect(&self, context: &Inspection<'_>) -> EcoString {
            self.value.inspect(context)
        }
    }

    #[geam::function]
    fn key<Identity>(#[geam::call] call: &mut Call<()>, identity: Value<Identity>) -> Key {
        Key {
            value: call.store_dynamic::<_, Key>(identity).native_view(),
        }
    }

    #[geam::function]
    fn send<Message>(
        #[geam::call] call: &mut Call<()>,
        target: service::Pid,
        message: Value<Message>,
    ) -> HostResult<()> {
        call.send(&target, message)
    }

    #[geam::function(await, profile = Profile)]
    async fn receive<Identity>(
        #[geam::call] call: &mut Call<()>,
        #[geam::restore] restore: Restore<service::Reference>,
        identity: Value<Identity>,
        forever: bool,
    ) -> HostResult<Result<(StringValue, service::Reference, BigInt), ()>> {
        let receive = call
            .with_call(move |call| {
                let identity = call.store_dynamic::<_, Key>(identity).native_view();
                let capture = Selection {
                    identity,
                    visits: Cell::new(0),
                };
                call.receive_with(
                    move |values, candidate| capture.select(values, candidate),
                    Some(Duration::ZERO),
                )
            })
            .await??;
        let selected = if forever {
            Some(receive.wait_forever_in(call).await?)
        } else {
            receive.wait_in(call).await?
        };
        call.with_call(move |call| match selected {
            Some(selected) => Ok((|| {
                Some((
                    selected.tag,
                    call.restore_native::<service::Reference>(&restore, &selected.identity)?,
                    selected.payload.as_int()?,
                ))
            })()
            .ok_or(())),
            None => Ok(Err(())),
        })
        .await?
    }

    #[geam::function(await, profile = Profile)]
    async fn next(
        #[geam::call] call: &mut Call<()>,
        #[geam::restore] restore: Restore<service::Reference>,
    ) -> HostResult<Result<(StringValue, service::Reference, BigInt), ()>> {
        let receive = call
            .with_call(|call| call.receive_any(Some(Duration::ZERO)))
            .await??;
        let selected = receive
            .wait_in(call)
            .await?
            .and_then(|message| Message::read(&message));
        call.with_call(move |call| match selected {
            Some(selected) => Ok((|| {
                Some((
                    selected.tag,
                    call.restore_native::<service::Reference>(&restore, &selected.identity)?,
                    selected.payload.as_int()?,
                ))
            })()
            .ok_or(())),
            None => Ok(Err(())),
        })
        .await?
    }

    // No Clone implementation, and Cell makes this Send-only capture non-Sync.
    struct Selection {
        identity: NativeValue,
        visits: Cell<usize>,
    }

    struct Message {
        tag: StringValue,
        identity: NativeValue,
        payload: NativeValue,
    }

    impl Message {
        fn read(candidate: &NativeValue) -> Option<Self> {
            match (
                candidate.kind(),
                candidate.len(),
                candidate.index(0),
                candidate.index(1),
                candidate.index(2),
            ) {
                (NativeKind::Tuple, Some(3), Some(tag), Some(identity), Some(payload)) => {
                    Some(Self {
                        tag: tag.as_string()?,
                        identity,
                        payload,
                    })
                }
                _ => None,
            }
        }
    }

    impl Selection {
        fn select(&self, values: NativeValues<'_>, candidate: &NativeValue) -> Option<Message> {
            self.visits.set(self.visits.get() + 1);
            let message = Message::read(candidate)?;
            if !matches!(
                message.tag.as_str(),
                Ok("tcp" | "closed" | "passive" | "error")
            ) {
                return None;
            }
            values
                .equal(&self.identity, &message.identity)
                .then_some(message)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::test_profile::{State, providers};
        use geam::execution::TokioHost;
        use geam::gleam_erlang::Configuration;
        use geam::gleam_stdlib::GleamStdlibRunState;
        use geam::{HostedExecution, compile_typed_host_project, plan_host_program};

        #[test]
        fn retained_keys_delegate_equality_hashing_and_inspection_to_original_values() {
            let directory = tempfile::tempdir().unwrap();
            let project = camino::Utf8Path::from_path(directory.path()).unwrap();
            std::fs::create_dir_all(project.join("src/selective_receive_service_fixture")).unwrap();
            std::fs::write(
                project.join("gleam.toml"),
                r#"
name = "selective_receive_service_fixture"
version = "0.1.0"
[dependencies]
gleam_erlang = ">= 1.3.0 and < 1.3.1"
gleam_stdlib = ">= 1.0.3 and < 1.0.4"
"#,
            )
            .unwrap();
            std::fs::copy(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../project/manifest.toml"),
                project.join("manifest.toml"),
            )
            .unwrap();
            std::fs::write(
                project.join("src/selective_receive_service_fixture/native.gleam"),
                r#"
import gleam/erlang/process.{type Pid}
import gleam/erlang/reference.{type Reference}
pub type Key
@external(erlang, "fixture", "key") pub fn key(identity: a) -> Key
@external(erlang, "fixture", "send") pub fn send(target: Pid, message: a) -> Nil
@external(erlang, "fixture", "receive")
pub fn receive(
  identity: a,
  forever: Bool,
) -> Result(#(String, Reference, Int), Nil)
@external(erlang, "fixture", "next") pub fn next() -> Result(#(String, Reference, Int), Nil)
"#,
            )
            .unwrap();
            std::fs::write(
                project.join("src/selective_receive_service_fixture.gleam"),
                r#"
import gleam/dict
import gleam/erlang/process
import gleam/erlang/reference
import selective_receive_service_fixture/native
pub fn main() {
  let a = reference.new()
  let b = reference.new()
  let key = native.key(a)
  let alias = native.key(a)
  let other = native.key(b)
  let keys = dict.from_list([#(key, 11), #(other, 22)])
  let strings = dict.from_list([#(native.key("key"), 33)])
  native.send(process.self(), #("tcp", a, 42))
  let assert Ok(#("tcp", delivered, payload)) = native.next()
  echo native.key("key")
  #(
    key == alias,
    key == other,
    dict.get(keys, alias),
    dict.get(keys, other),
    dict.get(strings, native.key("key")),
    native.key(delivered) == key,
    payload,
  )
}
"#,
            )
            .unwrap();
            let acquired = std::process::Command::new("gleam")
                .args(["deps", "download"])
                .current_dir(project)
                .status()
                .unwrap();
            assert_eq!(acquired.code(), Some(0));
            let typed = compile_typed_host_project(
                project,
                "selective_receive_service_fixture",
                providers(),
            )
            .unwrap();
            let mut execution =
                HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
            let executor = tokio::runtime::Builder::new_current_thread()
                .enable_time()
                .build()
                .unwrap();
            let host = TokioHost::new(executor.handle().clone());
            let mut state = State {
                stdlib: GleamStdlibRunState::from_seed([0; 32]),
                erlang: Configuration::default(),
                provider: (),
            };
            let mut echo = Vec::new();
            let value = executor
                .block_on(execution.run_main(&host, &mut state, &mut echo))
                .unwrap()
                .try_into_value()
                .unwrap();
            assert_eq!(
                value.inspect().to_string(),
                "#(True, False, Ok(11), Ok(22), Ok(33), True, 42)"
            );
            drop(execution);
            drop(state);
            assert_eq!(echo.len(), 1);
            assert_eq!(echo[0].value().inspect().to_string(), "\"key\"");
        }
    }
}
