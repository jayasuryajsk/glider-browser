use std::process::Command;
use std::time::Duration;
use std::thread::sleep;

#[test]
fn smoke() {
    let mut child = Command::new("cargo")
        .args(["run", "-p", "desktop_launcher", "--", "--headless", "about:blank"])
        .spawn()
        .expect("failed to run desktop_launcher");

    sleep(Duration::from_secs(3));
    let status = child.wait().expect("failed to wait");
    assert!(status.success());
}
