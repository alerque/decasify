#[cfg(feature = "cli")]
use {assert_cmd::Command, predicates::prelude::*};

#[cfg(feature = "cli")]
#[test]
fn main() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(["foo"])
        .assert()
        .success()
        .stdout("Foo\n")
        .stderr("");
}

#[cfg(feature = "cli")]
#[test]
fn main_stdin() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.write_stdin("foo")
        .assert()
        .success()
        .stdout("Foo\n")
        .stderr("");
}

#[cfg(feature = "cli")]
#[test]
fn main_help() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage: "))
        .stderr("");
}

#[cfg(feature = "cli")]
#[test]
fn main_lang() {
    let mut en_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    en_cmd
        .args(["-l", "en", "ide"])
        .assert()
        .success()
        .stdout("Ide\n")
        .stderr("");
    let mut tr_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    tr_cmd
        .args(["-l", "tr", "ilk"])
        .assert()
        .success()
        .stdout("İlk\n")
        .stderr("");
}
