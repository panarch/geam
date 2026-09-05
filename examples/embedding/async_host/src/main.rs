mod geam_bindings;
mod host;

use futures::executor::block_on;
use geam::embedding::AsyncHostedModuleBuilder;
use geam::{EchoOutput, EchoSink};

#[derive(Default)]
struct TextEcho(Vec<String>);

impl EchoSink for TextEcho {
    fn emit(&mut self, output: EchoOutput) {
        let message = output.message().map_or("echo", |message| message.as_str());
        self.0
            .push(format!("{message}: {}", output.value().inspect()));
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = geam_bindings::project()
        .with_async_hosts(host::async_hosts()?)
        .compile()?;
    let builder = AsyncHostedModuleBuilder::new(program)?;
    let (bindings, functions) = geam_bindings::bind_async(builder)?;
    let mut module = bindings.seal();
    let mut state = host::RunState::new(2);
    let mut echo = TextEcho::default();

    let value =
        block_on(module.call_async(&functions.calculate, (20.into(),), &mut state, &mut echo))?;

    println!("value: {value}");
    println!("completed: {}", state.completed());
    for output in echo.0 {
        println!("{output}");
    }
    Ok(())
}
