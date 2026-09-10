use std::io::Write;
use std::process::{Command, Stdio};

pub fn preprocess(source: &str) -> String {
    let mut child = Command::new("gcc")
        .args(["-E", "-P", "-x", "c", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start gcc for preprocessing. Is gcc installed?\n");

    child
        .stdin
        .take()
        .expect("Failed to open stdin for gcc.\n")
        .write_all(source.as_bytes())
        .expect("Failed to write source to gcc's stdin.\n");

    let output = child
        .wait_with_output()
        .expect("Failed to read gcc's output.\n");

    if !output.status.success() {
        panic!(
            "gcc preprocessing failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    String::from_utf8(output.stdout).expect("gcc produced non-UTF-8 output.\n")
}
