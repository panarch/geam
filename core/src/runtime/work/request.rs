use super::Cancelled;
use futures_channel::oneshot;
use parking_lot::Mutex;
use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak};
use std::task::{Context, Poll, Waker};

pub(crate) struct Requests<Request> {
    queue: Arc<Mutex<Queue<Request>>>,
}

pub(crate) struct Sender<Request> {
    queue: Weak<Mutex<Queue<Request>>>,
}

pub(crate) struct Submitted<Request, Output> {
    message: Arc<Message<Request>>,
    response: oneshot::Receiver<Output>,
}

pub(crate) type Reply<Output> = oneshot::Sender<Output>;

struct Queue<Request> {
    accepting: bool,
    messages: BTreeMap<u64, Weak<Message<Request>>>,
    driver: Option<Arc<Waker>>,
}

struct Message<Request> {
    identity: u64,
    queue: Weak<Mutex<Queue<Request>>>,
    request: Mutex<Option<Request>>,
}

impl<Request> Requests<Request> {
    pub(crate) fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(Queue {
                accepting: true,
                messages: BTreeMap::new(),
                driver: None,
            })),
        }
    }

    pub(crate) fn sender(&self) -> Sender<Request> {
        Sender {
            queue: Arc::downgrade(&self.queue),
        }
    }

    pub(crate) fn next(&self, cx: &mut Context<'_>) -> Option<Request> {
        for _ in 0..64 {
            let waker = Arc::new(cx.waker().clone());
            let (entry, previous) = {
                let mut queue = self.queue.lock();
                (queue.messages.pop_first(), queue.driver.replace(waker))
            };
            drop(previous);
            let (_, message) = entry?;
            if let Some(message) = message.upgrade() {
                let request = message.request.lock().take();
                if request.is_some() {
                    return request;
                }
            }
        }
        cx.waker().wake_by_ref();
        None
    }

    pub(crate) fn close(&self) {
        let (messages, driver) = {
            let mut queue = self.queue.lock();
            queue.accepting = false;
            (std::mem::take(&mut queue.messages), queue.driver.take())
        };
        for message in messages
            .into_values()
            .filter_map(|message| message.upgrade())
        {
            let request = message.request.lock().take();
            drop(request);
        }
        drop(driver);
    }
}

impl<Request> Drop for Requests<Request> {
    fn drop(&mut self) {
        self.close();
    }
}

impl<Request> Sender<Request> {
    pub(crate) fn same_queue(&self, other: &Self) -> bool {
        self.queue.ptr_eq(&other.queue)
    }

    pub(crate) fn submit<Output>(
        &self,
        request: impl FnOnce(Reply<Output>) -> Request,
    ) -> Submitted<Request, Output> {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let (reply, response) = oneshot::channel();
        let message = Arc::new(Message {
            identity: NEXT.fetch_add(1, Ordering::Relaxed),
            queue: self.queue.clone(),
            request: Mutex::new(Some(request(reply))),
        });
        let queued = self.queue.upgrade().and_then(|queue| {
            let mut queue = queue.lock();
            if queue.accepting {
                queue
                    .messages
                    .insert(message.identity, Arc::downgrade(&message));
                Some(queue.driver.clone())
            } else {
                None
            }
        });
        match queued {
            Some(driver) => {
                if let Some(driver) = driver {
                    driver.wake_by_ref();
                }
            }
            None => {
                let request = message.request.lock().take();
                drop(request);
            }
        }
        Submitted { message, response }
    }
}

impl<Request> Clone for Sender<Request> {
    fn clone(&self) -> Self {
        Self {
            queue: self.queue.clone(),
        }
    }
}

impl<Request, Output> Future for Submitted<Request, Output> {
    type Output = Result<Output, Cancelled>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.response)
            .poll(cx)
            .map(|result| result.map_err(|_| Cancelled))
    }
}

impl<Request, Output> Drop for Submitted<Request, Output> {
    fn drop(&mut self) {
        let request = self.message.request.lock().take();
        drop(request);
    }
}

impl<Request> Drop for Message<Request> {
    fn drop(&mut self) {
        if let Some(queue) = self.queue.upgrade() {
            queue.lock().messages.remove(&self.identity);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Cancelled, Reply, Requests};
    use std::cell::Cell;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::{Context, Poll, Wake, Waker};

    struct Request {
        input: Input,
        reply: Reply<Cell<usize>>,
    }

    fn submit_input(
        sender: &super::Sender<Request>,
        drops: &Arc<AtomicUsize>,
    ) -> super::Submitted<Request, Cell<usize>> {
        sender.submit(|reply| Request {
            input: Input(Arc::clone(drops)),
            reply,
        })
    }

    struct PauseDrop(Arc<std::sync::Barrier>);

    impl Drop for PauseDrop {
        fn drop(&mut self) {
            self.0.wait();
            self.0.wait();
        }
    }

    struct Input(Arc<AtomicUsize>);

    impl Drop for Input {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[derive(Default)]
    struct Notifications(AtomicUsize);

    impl Wake for Notifications {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn native_requests_are_owned_until_the_original_driver_services_them() {
        let requests = Requests::new();
        let sender = requests.sender();
        let drops = Arc::new(AtomicUsize::new(0));
        let notifications = Arc::new(Notifications::default());
        let waker = Waker::from(Arc::clone(&notifications));
        let mut cx = Context::from_waker(&waker);
        assert!(requests.next(&mut cx).is_none());
        let mut response = submit_input(&sender, &drops);
        assert_eq!(notifications.0.load(Ordering::Relaxed), 1);
        assert!(Pin::new(&mut response).poll(&mut cx).is_pending());
        assert_eq!(drops.load(Ordering::Relaxed), 0);

        let Request { input, reply } = requests.next(&mut cx).unwrap();
        assert!(!reply.is_canceled());
        drop(input);
        assert_eq!(reply.send(Cell::new(42)), Ok(()));
        assert_eq!(notifications.0.load(Ordering::Relaxed), 2);
        assert_eq!(
            Pin::new(&mut response)
                .poll(&mut cx)
                .map(|value| value.map(|cell| cell.get())),
            Poll::Ready(Ok(42))
        );
        assert_eq!(drops.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn abandoning_a_request_releases_its_input_without_another_driver_poll() {
        let requests = Requests::new();
        let sender = requests.sender();
        let drops = Arc::new(AtomicUsize::new(0));
        for _ in 0..100 {
            let response = submit_input(&sender, &drops);
            drop(response);
        }
        assert_eq!(drops.load(Ordering::Relaxed), 100);
        let pending = requests.queue.lock().messages.len();
        assert_eq!(pending, 0);
        assert!(
            requests
                .next(&mut Context::from_waker(Waker::noop()))
                .is_none()
        );
    }

    #[test]
    fn shutdown_rejects_a_sender_whose_queue_reservation_outlives_the_receiver() {
        let requests = Requests::new();
        let sender = requests.sender();
        let in_flight = sender.queue.upgrade().expect("live sender reservation");
        drop(requests);
        let drops = Arc::new(AtomicUsize::new(0));
        let mut response = submit_input(&sender, &drops);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(
            Pin::new(&mut response)
                .poll(&mut Context::from_waker(Waker::noop()))
                .map(Result::err),
            Poll::Ready(Some(Cancelled))
        );
        assert!(in_flight.lock().messages.is_empty());
        drop(response);
        drop(in_flight);
    }

    #[test]
    fn a_cancelling_request_does_not_hide_the_next_live_request() {
        use std::sync::Barrier;

        let requests = Requests::new();
        let sender = requests.sender();
        let barrier = Arc::new(Barrier::new(2));
        let cancelled =
            sender.submit(|reply: Reply<usize>| (Some(PauseDrop(Arc::clone(&barrier))), reply));
        let mut live = sender.submit(|reply| (None, reply));
        let cancelling = std::thread::spawn(move || drop(cancelled));
        barrier.wait();

        let mut cx = Context::from_waker(Waker::noop());
        let request = requests.next(&mut cx);
        barrier.wait();
        cancelling.join().unwrap();
        let (input, reply) = request.unwrap();
        assert!(input.is_none());
        reply.send(42).unwrap();
        assert_eq!(Pin::new(&mut live).poll(&mut cx), Poll::Ready(Ok(42)));
        assert!(requests.next(&mut cx).is_none());
    }

    #[test]
    fn a_full_turn_of_cancelling_requests_yields_before_servicing_the_live_tail() {
        use std::sync::Barrier;

        let requests = Requests::new();
        let sender = requests.sender();
        let barrier = Arc::new(Barrier::new(65));
        let notifications = Arc::new(Notifications::default());
        let waker = Waker::from(Arc::clone(&notifications));
        let mut cx = Context::from_waker(&waker);
        std::thread::scope(|threads| {
            for _ in 0..64 {
                let submitted = sender
                    .submit(|reply: Reply<usize>| (Some(PauseDrop(Arc::clone(&barrier))), reply));
                threads.spawn(move || drop(submitted));
            }
            let mut live = sender.submit(|reply| (None, reply));
            barrier.wait();
            // Every cancelled request is empty but still retained by its destructor.
            assert!(requests.next(&mut cx).is_none());
            assert_eq!(notifications.0.load(Ordering::Relaxed), 1);
            let (input, reply) = requests.next(&mut cx).unwrap();
            assert!(input.is_none());
            reply.send(42).unwrap();
            assert_eq!(Pin::new(&mut live).poll(&mut cx), Poll::Ready(Ok(42)));
            barrier.wait();
        });
        assert!(requests.next(&mut cx).is_none());
    }

    #[test]
    fn closing_an_endpoint_cancels_pending_and_later_requests() {
        let requests = Requests::new();
        let sender = requests.sender();
        let drops = Arc::new(AtomicUsize::new(0));
        let mut pending = submit_input(&sender, &drops);
        drop(requests);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        assert_eq!(
            Pin::new(&mut pending)
                .poll(&mut Context::from_waker(Waker::noop()))
                .map(Result::err),
            Poll::Ready(Some(Cancelled))
        );
        let mut later = submit_input(&sender.clone(), &drops);
        assert_eq!(drops.load(Ordering::Relaxed), 2);
        assert_eq!(
            Pin::new(&mut later)
                .poll(&mut Context::from_waker(Waker::noop()))
                .map(Result::err),
            Poll::Ready(Some(Cancelled))
        );
    }

    #[test]
    fn already_claimed_requests_observe_receiver_cancellation() {
        let requests = Requests::new();
        let sender = requests.sender();
        let drops = Arc::new(AtomicUsize::new(0));
        let response = submit_input(&sender, &drops);
        let request = requests
            .next(&mut Context::from_waker(Waker::noop()))
            .unwrap();
        drop(response);
        assert!(request.reply.is_canceled());
        drop(request);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn a_message_dropped_after_dequeue_does_not_hide_the_next_live_request() {
        struct PausingWaker {
            barrier: Arc<std::sync::Barrier>,
            wakes: Arc<AtomicUsize>,
        }
        impl Wake for PausingWaker {
            fn wake(self: Arc<Self>) {
                self.wakes.fetch_add(1, Ordering::SeqCst);
            }
        }
        impl Drop for PausingWaker {
            fn drop(&mut self) {
                self.barrier.wait();
                self.barrier.wait();
            }
        }
        let requests = Requests::new();
        let sender = requests.sender();
        let barrier = Arc::new(std::sync::Barrier::new(2));
        let wakes = Arc::new(AtomicUsize::new(0));
        let waker = Waker::from(Arc::new(PausingWaker {
            barrier: Arc::clone(&barrier),
            wakes: Arc::clone(&wakes),
        }));
        assert!(requests.next(&mut Context::from_waker(&waker)).is_none());
        drop(waker);
        let cancelled = sender.submit(|reply: Reply<usize>| (None::<PauseDrop>, reply));
        let mut live = sender.submit(|reply: Reply<usize>| (None::<PauseDrop>, reply));
        assert_eq!(wakes.load(Ordering::SeqCst), 2);
        let request = std::thread::scope(|threads| {
            let worker = threads.spawn(|| requests.next(&mut Context::from_waker(Waker::noop())));
            // Replacing the previous Waker pauses after removing the first weak entry.
            barrier.wait();
            drop(cancelled);
            barrier.wait();
            worker.join().expect("dequeue worker")
        })
        .expect("live request after expired entry");
        let (input, reply) = request;
        assert!(input.is_none());
        reply.send(42).expect("live receiver");
        let mut cx = Context::from_waker(Waker::noop());
        assert_eq!(Pin::new(&mut live).poll(&mut cx), Poll::Ready(Ok(42)));
        assert!(requests.next(&mut cx).is_none());
    }
}
