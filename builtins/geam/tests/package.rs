use serde_json::{Value, json};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, Receiver};
use std::time::Duration;
use url::Url;

const SOURCE: &str = "import geam/future.{type Future}\nimport geam_package_helper\n\npub fn main() -> Future(Int) {\n  future.map(geam_package_helper.answer(), fn(value) { value * 2 })\n}\n";

struct PackageFixture {
    _root: tempfile::TempDir,
    app: PathBuf,
    api: PathBuf,
}

impl PackageFixture {
    fn new() -> Self {
        let root = tempfile::tempdir().expect("package fixture");
        let api = Path::new(env!("CARGO_MANIFEST_DIR")).join("gleam");
        let app = root.path().join("app with spaces");
        let helper = root.path().join("helper");
        std::fs::create_dir_all(app.join("src")).expect("app source directory");
        std::fs::create_dir_all(helper.join("src")).expect("helper source directory");
        std::fs::write(helper.join("gleam.toml"), format!(
            "name = \"geam_package_helper\"\nversion = \"0.0.0\"\ntarget = \"erlang\"\n[dependencies]\ngeam = {{ path = {:?} }}\n", api
        )).expect("helper dependency");
        std::fs::write(helper.join("src/geam_package_helper.gleam"),
            "import geam/future.{type Future}\n\npub fn answer() -> Future(Int) {\n  future.ready(21)\n}\n"
        ).expect("helper source");
        std::fs::write(app.join("gleam.toml"), format!(
            "name = \"geam_package_consumer\"\nversion = \"0.0.0\"\ntarget = \"erlang\"\n[dependencies]\ngeam = {{ path = {:?} }}\ngeam_package_helper = {{ path = {:?} }}\n", api, helper
        )).expect("application dependencies");
        std::fs::write(app.join("src/main.gleam"), SOURCE).expect("application source");
        Self {
            _root: root,
            app,
            api,
        }
    }

    fn build(&self) {
        let output = Command::new("gleam")
            .arg("build")
            .current_dir(&self.app)
            .output()
            .expect("official Gleam CLI");
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn ordinary_shared_dependency_builds_and_erlang_reports_the_runtime_requirement() {
    let fixture = PackageFixture::new();
    fixture.build();
    let mut command = Command::new("erl");
    command.arg("-noshell");
    for package in ["geam", "geam_package_helper", "geam_package_consumer"] {
        command.arg("-pa").arg(
            fixture
                .app
                .join("build/dev/erlang")
                .join(package)
                .join("ebin"),
        );
    }
    let output = command.args(["-eval", r#"
Message = <<"geam/future is currently implemented by Geam. An Erlang implementation is not currently available.">>,
Cases = [
  fun() -> geam_future:ready(1) end,
  fun() -> geam_future:map(unused, fun(X) -> X end) end,
  fun() -> geam_future:flatten(unused) end,
  fun() -> geam_future:all([]) end,
  fun() -> 'geam@future':ready(1) end,
  fun() -> 'geam@future':map(unused, fun(X) -> X end) end,
  fun() -> 'geam@future':then(unused, fun(X) -> X end) end,
  fun() -> 'geam@future':all([]) end,
  fun main:main/0
],
lists:foreach(fun(Run) ->
  try Run() of _ -> erlang:error(unexpected_success)
  catch error:Message -> io:format("expected runtime diagnostic~n")
  end
end, Cases),
halt(0).
"#]).output().expect("Erlang runtime");
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).expect("Erlang stdout"),
        "expected runtime diagnostic\n".repeat(9)
    );
}

struct LanguageServer {
    child: Child,
    input: ChildStdin,
    messages: Receiver<Value>,
    diagnostics: Vec<Value>,
    next_id: u32,
}

impl LanguageServer {
    fn start(root: &Path) -> Self {
        let mut child = Command::new("gleam")
            .arg("lsp")
            .current_dir(root)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("official Gleam language server");
        let input = child.stdin.take().expect("LSP stdin");
        let output = child.stdout.take().expect("LSP stdout");
        let (send, messages) = mpsc::channel();
        std::thread::spawn(move || {
            let mut reader = BufReader::new(output);
            loop {
                let mut length = None;
                loop {
                    let mut header = String::new();
                    if reader.read_line(&mut header).expect("LSP header") == 0 {
                        return;
                    }
                    if header == "\r\n" {
                        break;
                    }
                    if let Some(value) = header.strip_prefix("Content-Length:") {
                        length = Some(value.trim().parse::<usize>().expect("LSP content length"));
                    }
                }
                let mut body = vec![0; length.expect("LSP content header")];
                reader.read_exact(&mut body).expect("LSP message body");
                if send
                    .send(serde_json::from_slice(&body).expect("LSP JSON"))
                    .is_err()
                {
                    return;
                }
            }
        });
        Self {
            child,
            input,
            messages,
            diagnostics: Vec::new(),
            next_id: 0,
        }
    }

    fn send(&mut self, value: Value) {
        let bytes = serde_json::to_vec(&value).expect("LSP request JSON");
        write!(self.input, "Content-Length: {}\r\n\r\n", bytes.len()).expect("LSP request header");
        self.input.write_all(&bytes).expect("LSP request body");
        self.input.flush().expect("LSP request flush");
    }

    fn notify(&mut self, method: &str, params: Value) {
        self.send(json!({"jsonrpc": "2.0", "method": method, "params": params}));
    }

    fn request(&mut self, method: &str, params: Value) -> Value {
        self.next_id += 1;
        let id = self.next_id;
        self.send(json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params}));
        loop {
            let message = self
                .messages
                .recv_timeout(Duration::from_secs(60))
                .expect("LSP response deadline");
            if message["method"] == "textDocument/publishDiagnostics" {
                self.diagnostics.push(message["params"].clone());
            }
            if message["id"] == id {
                assert!(message.get("error").is_none(), "{method}: {message}");
                return message["result"].clone();
            }
        }
    }

    fn errors(&self) -> Vec<&Value> {
        self.diagnostics
            .iter()
            .flat_map(|params| params["diagnostics"].as_array().expect("diagnostics array"))
            .filter(|diagnostic| diagnostic["severity"] == 1)
            .collect()
    }
}

impl Drop for LanguageServer {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[test]
fn official_editor_resolves_future_and_reports_then_clears_a_type_error() {
    let fixture = PackageFixture::new();
    fixture.build();
    let mut server = LanguageServer::start(&fixture.app);
    let uri = Url::from_file_path(fixture.app.join("src/main.gleam"))
        .expect("source URI")
        .to_string();
    server.request(
        "initialize",
        json!({
            "processId": null,
            "rootUri": Url::from_directory_path(&fixture.app).expect("project URI").as_str(),
            "capabilities": {},
        }),
    );
    server.notify("initialized", json!({}));
    server.notify(
        "textDocument/didOpen",
        json!({"textDocument": {
            "uri": uri, "languageId": "gleam", "version": 1, "text": SOURCE,
        }}),
    );
    let position = json!({"textDocument": {"uri": uri}, "position": {"line": 4, "character": 10}});
    let hover = server.request("textDocument/hover", position.clone());
    assert!(hover.to_string().contains("Future"), "{hover}");
    assert!(server.errors().is_empty(), "{:?}", server.diagnostics);
    let definition = server.request("textDocument/definition", position.clone());
    assert!(
        definition
            .to_string()
            .contains("/geam/src/geam/future.gleam")
            || definition
                .to_string()
                .contains("/gleam/src/geam/future.gleam"),
        "{definition}"
    );
    assert!(fixture.api.join("src/geam/future.gleam").is_file());
    let completions = server.request("textDocument/completion", position.clone());
    assert!(completions.to_string().contains("map"), "{completions}");

    for (version, source, has_error) in [
        (2, SOURCE.replace("value * 2", "value <> \"wrong\""), true),
        (3, SOURCE.to_owned(), false),
    ] {
        server.diagnostics.clear();
        std::fs::write(fixture.app.join("src/main.gleam"), &source).expect("edited source");
        server.notify("textDocument/didChange", json!({"textDocument": {"uri": uri, "version": version}, "contentChanges": [{"text": source}]}));
        server.notify(
            "textDocument/didSave",
            json!({"textDocument": {"uri": uri}}),
        );
        server.request("textDocument/hover", position.clone());
        assert!(
            !server.diagnostics.is_empty(),
            "the editor must publish diagnostics after save"
        );
        assert_eq!(
            !server.errors().is_empty(),
            has_error,
            "{:?}",
            server.diagnostics
        );
        if has_error {
            assert!(
                server.errors().iter().any(|error| error["message"]
                    .as_str()
                    .expect("diagnostic text")
                    .contains("type")),
                "{:?}",
                server.errors()
            );
        }
    }
    server.request("shutdown", Value::Null);
    server.notify("exit", Value::Null);
}
