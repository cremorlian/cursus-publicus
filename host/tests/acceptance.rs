#[test]
fn launches_and_shows_hello_world() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_cp-host"))
        .output()
        .expect("failed to run host app");
    let stdout = String::from_utf8(output.stdout).expect("stdout was not utf8");
    assert!(
        stdout.contains("Hello World"),
        "expected stdout to contain 'Hello World', got: {:?}",
        stdout
    );
}
