use std::cell::Cell;
use std::pin::Pin;
use std::sync::mpsc::Sender;
use std::time::Duration;
use tokio::time::Sleep;

pub struct State {
    timer: Option<Pin<Box<Sleep>>>,
    calls: Cell<usize>,
    release: Option<Sender<()>>,
}

impl Default for State {
    fn default() -> Self {
        println!("initialized");
        Self {
            timer: Some(Box::pin(tokio::time::sleep(Duration::from_secs(3600)))),
            calls: Cell::new(0),
            release: None,
        }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        assert!(tokio::runtime::Handle::try_current().is_ok());
        println!("state-drop:{}", self.calls.get());
        self.release.take();
    }
}

struct PendingWork;

impl Drop for PendingWork {
    fn drop(&mut self) {
        assert!(tokio::runtime::Handle::try_current().is_ok());
        println!("pending-drop");
    }
}

#[geam::provider(package = "standalone_future", state = State, modules = [native])]
pub struct Component;

#[geam::module(path = "standalone_future/native")]
mod native {
    use super::State;
    use geam::provider::{BigInt, Call, EcoString, HostFailure, HostResult};
    use std::future::{Future, poll_fn};
    use std::task::Poll;
    use std::time::Duration;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[geam::function]
    async fn timer(#[geam::call] call: &mut Call<State>) -> HostResult<BigInt> {
        let mut timer = call
            .with_state(|state| state.timer.take().expect("one timer"))
            .await?;
        poll_fn(|cx| {
            assert!(timer.as_mut().poll(cx).is_pending());
            println!("timer-pending");
            timer.as_mut().reset(tokio::time::Instant::now());
            Poll::Ready(())
        })
        .await;
        timer.await;
        let count = call
            .with_state(|state| {
                state.calls.set(state.calls.get() + 1);
                state.calls.get()
            })
            .await?;
        println!("timer-complete");
        Ok(count.into())
    }

    #[geam::function]
    fn current(#[geam::call] call: &Call<State>) -> BigInt {
        let count = call.state().calls.get();
        println!("state:{count}");
        count.into()
    }

    #[geam::function]
    async fn request(address: EcoString) -> HostResult<EcoString> {
        let mut stream = tokio::net::TcpStream::connect(address.as_str())
            .await
            .map_err(|error| HostFailure::new(error.to_string()))?;
        stream
            .write_all(b"ping")
            .await
            .map_err(|error| HostFailure::new(error.to_string()))?;
        let mut bytes = Vec::new();
        stream
            .read_to_end(&mut bytes)
            .await
            .map_err(|error| HostFailure::new(error.to_string()))?;
        Ok(String::from_utf8(bytes)
            .map_err(|error| HostFailure::new(error.to_string()))?
            .into())
    }

    #[geam::function]
    async fn fail() -> HostResult<()> {
        Err(HostFailure::new("native failure").into())
    }

    #[geam::function]
    async fn pending() -> HostResult<()> {
        let _work = super::PendingWork;
        println!("pending-started");
        std::future::pending().await
    }

    #[geam::function]
    async fn spawn_worker(#[geam::call] call: &mut Call<State>) -> HostResult<()> {
        let (release, released) = std::sync::mpsc::channel();
        call.with_state(move |state| {
            state.release = Some(release);
        })
        .await?;
        let (started, wait) = tokio::sync::oneshot::channel();
        tokio::task::spawn_blocking(move || {
            println!("blocking-started");
            started.send(()).expect("driver waiting");
            assert!(matches!(
                released.recv_timeout(Duration::from_secs(5)),
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected)
            ));
            println!("blocking-finished");
        });
        wait.await.expect("blocking task started");
        println!("entry-complete");
        Ok(())
    }
}
