//! Ordinary native consumption of the original standard-library BytesTree.

use futures_channel::oneshot;
use geam::gleam_stdlib::service;
use std::sync::{Arc, Mutex, Weak};

#[derive(Default)]
pub struct RunState {
    pub retained: Option<service::BytesTreeInput>,
    pub pending: Arc<Mutex<Weak<Mutex<service::BytesTreeInput>>>>,
    pub gate: Option<oneshot::Receiver<()>>,
}

#[geam::provider(
    package = "bytes_tree_service_fixture",
    state = RunState,
    modules = [native],
)]
pub struct Component;

#[geam::module(path = "bytes_tree_service_fixture")]
mod native {
    use super::{Arc, Mutex, RunState, service};
    use geam::provider::{BitArrayValue, Call, HostResult};
    use std::future::poll_fn;
    use std::task::Poll;

    #[geam::function]
    fn read(tree: service::BytesTreeInput) -> BitArrayValue {
        tree.to_bit_array()
    }

    #[geam::function]
    fn retain(#[geam::call] call: &mut Call<RunState>, tree: service::BytesTreeInput) -> () {
        call.state_mut().retained = Some(tree);
    }

    #[geam::function(await)]
    async fn read_after_pause(
        #[geam::call] call: &mut Call<RunState>,
        tree: service::BytesTreeInput,
    ) -> HostResult<(BitArrayValue, BitArrayValue)> {
        let before = tree.to_bit_array();
        let input = Arc::new(Mutex::new(tree));
        let retained = Arc::downgrade(&input);
        let gate = call
            .with_state(move |state| {
                *state.pending.lock().unwrap() = retained;
                state.gate.take()
            })
            .await?;
        if let Some(gate) = gate {
            let _ = gate.await;
        } else {
            let mut yielded = false;
            poll_fn(|cx| {
                if yielded {
                    Poll::Ready(())
                } else {
                    yielded = true;
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            })
            .await;
        }
        let after = input.lock().unwrap().to_bit_array();
        Ok((before, after))
    }
}
