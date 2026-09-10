use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub(in crate::runtime) struct Yield {
    yielded: bool,
}

impl Yield {
    pub(in crate::runtime) fn new() -> Self {
        Self { yielded: false }
    }
}

impl Future for Yield {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.yielded {
            Poll::Ready(())
        } else {
            self.yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Yield;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::{Context, Poll, Wake, Waker};

    struct Notices(AtomicUsize);
    impl Wake for Notices {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn a_turn_requests_one_later_poll_before_continuing() {
        let notices = Arc::new(Notices(AtomicUsize::new(0)));
        let waker = Waker::from(Arc::clone(&notices));
        let mut cx = Context::from_waker(&waker);
        let mut yielding = Yield::new();
        assert_eq!(Pin::new(&mut yielding).poll(&mut cx), Poll::Pending);
        assert_eq!(notices.0.load(Ordering::Relaxed), 1);
        assert_eq!(Pin::new(&mut yielding).poll(&mut cx), Poll::Ready(()));
        assert_eq!(notices.0.load(Ordering::Relaxed), 1);
    }
}
