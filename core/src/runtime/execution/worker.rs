use crate::execution::Worker;
use crate::runtime::work::Cancelled;
use crate::runtime::work::request::Reply;
use std::future::{Future, poll_fn};
use std::pin::pin;
use std::task::Poll;

pub(super) fn completing<Output: Send + 'static>(
    mut reply: Reply<Output>,
    operation: impl Future<Output = Result<Output, Cancelled>> + Send + 'static,
) -> Worker {
    Box::pin(async move {
        let output = {
            let mut operation = pin!(operation);
            poll_fn(|cx| {
                if reply.poll_canceled(cx).is_ready() {
                    return Poll::Ready(Err(Cancelled));
                }
                operation.as_mut().poll(cx)
            })
            .await
        };
        if let Ok(output) = output {
            let _ = reply.send(output);
        }
    })
}
