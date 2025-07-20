use std::process::Command;

#[test]
fn test_extension_runs() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute extension");
    
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("PM Extension"));
}
