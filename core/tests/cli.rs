use std::process::Command;

fn validator() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ndx-validate"))
}

#[test]
fn accepts_valid_shared_fixture() {
    let output = validator()
        .arg("../tests/fixtures/valid-minimal.ndx.json")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("run validator");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("VALID"));
}

#[test]
fn rejects_invalid_shared_fixture_with_nonzero_exit() {
    let output = validator()
        .arg("../tests/fixtures/invalid-duplicate-ids.ndx.json")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("run validator");

    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("NDX_E_DUPLICATE_ID"));
}

#[test]
fn reports_usage_error_when_path_is_missing() {
    let output = validator().output().expect("run validator");
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("usage:"));
}
