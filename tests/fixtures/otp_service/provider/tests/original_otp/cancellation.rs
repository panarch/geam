use super::support::{Observation, Profile, State, execution, execution_fixture::TestHost};
use geam::HostedExecution;
use std::future::Future;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock, mpsc};
use std::task::{Context, Poll, Wake, Waker};
use std::thread::ThreadId;
use std::time::Duration;

enum Event {
    Paused,
    TurnFinished,
}

/// Pauses a worker's notification before the submitted service response is polled.
/// The driver can then cancel while the native poll is still running.
struct Handoff {
    worker: OnceLock<ThreadId>,
    count: AtomicUsize,
    checkpoint: usize,
    events: mpsc::Sender<Event>,
    resume: Mutex<mpsc::Receiver<()>>,
}

impl Wake for Handoff {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        if self.worker.get() == Some(&std::thread::current().id())
            && self.count.fetch_add(1, Ordering::Relaxed) == self.checkpoint
        {
            self.events.send(Event::Paused).unwrap();
            self.resume
                .lock()
                .unwrap()
                .recv_timeout(Duration::from_secs(10))
                .unwrap();
        }
    }
}

#[test]
fn cancellation_at_each_host_handoff_releases_original_otp_processes() {
    for (entry, tick) in [
        ("otp_service_fixture", Duration::ZERO),
        ("otp_supervision", Duration::ZERO),
        ("otp_lifecycle", Duration::ZERO),
        ("otp_shutdown_timeout", Duration::from_millis(1)),
    ] {
        let (mut execution, mut state) = execution(entry);
        let (handoffs, cancelled) = run_at(&mut execution, &mut state, usize::MAX, tick);
        assert!(!cancelled);
        assert!(handoffs > 0);
        let mut cancellations = 0;
        for checkpoint in 0..handoffs {
            cancellations += usize::from(run_at(&mut execution, &mut state, checkpoint, tick).1);
        }
        assert!(cancellations > 0);
        eprintln!(
            "{entry}: checked {handoffs} host handoffs, {cancellations} active cancellations"
        );
    }
}

fn run_at(
    execution: &mut HostedExecution<Profile>,
    state: &mut State,
    checkpoint: usize,
    tick: Duration,
) -> (usize, bool) {
    state.observed = Arc::new(Mutex::new(Observation::default()));
    state.stdlib = geam::gleam_stdlib::GleamStdlibRunState::from_seed([0; 32]);
    let observed = Arc::clone(&state.observed);
    let host = TestHost::default();
    let (events, received) = mpsc::channel();
    let (resume, resumed) = mpsc::channel();
    let handoff = Arc::new(Handoff {
        worker: OnceLock::new(),
        count: AtomicUsize::new(0),
        checkpoint,
        events: events.clone(),
        resume: Mutex::new(resumed),
    });
    let waker = Waker::from(Arc::clone(&handoff));
    let mut cx = Context::from_waker(&waker);
    let mut echo = Vec::new();
    let mut running = Box::pin(execution.run_main(&host, state, &mut echo));
    let mut cancelled = false;
    let result = std::thread::scope(|threads| {
        let (turn, turns) = mpsc::channel();
        let worker = threads.spawn(|| {
            handoff.worker.set(std::thread::current().id()).unwrap();
            for () in turns {
                host.step();
                events.send(Event::TurnFinished).unwrap();
            }
        });
        let mut completed = None;
        let mut turns = 0;
        let result = loop {
            turns += 1;
            assert!(
                turns <= 4096,
                "OTP operation made no progress at handoff {checkpoint}"
            );
            if let Some(result) = completed.take() {
                break result;
            }
            if let Poll::Ready(result) = running.as_mut().poll(&mut cx) {
                break result;
            }
            host.advance(tick);
            turn.send(()).unwrap();
            match received.recv_timeout(Duration::from_secs(10)).unwrap() {
                Event::TurnFinished => {}
                Event::Paused => {
                    let units = observed.lock().unwrap().units.clone();
                    for unit in units {
                        cancelled |= unit.cancel();
                    }
                    if let Poll::Ready(result) = running.as_mut().poll(&mut cx) {
                        completed = Some(result);
                    }
                    resume.send(()).unwrap();
                    assert_eq!(
                        std::mem::discriminant(
                            &received.recv_timeout(Duration::from_secs(10)).unwrap()
                        ),
                        std::mem::discriminant(&Event::TurnFinished),
                    );
                }
            }
        };
        drop(turn);
        worker.join().unwrap();
        result
    });
    drop(running);
    let expected = cancelled.then(|| geam::execution::RunError::Cancelled.to_string());
    assert_eq!(result.err().map(|error| error.to_string()), expected);
    assert!(echo.is_empty());
    let observed = observed.lock().unwrap();
    assert_eq!(observed.closed, 1);
    assert_eq!(observed.finished.len(), observed.units.len());
    assert!(observed.units.iter().all(|unit| !unit.is_active()));
    (handoff.count.load(Ordering::Relaxed), cancelled)
}
