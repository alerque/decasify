use assert_cmd::Command;

#[cfg(feature = "cli")]
#[test]
fn main() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(["foo"]).assert().success().stdout("Foo\n").stderr("");
}

#[cfg(feature = "cli")]
#[test]
fn main_stdin() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.write_stdin("foo").assert().success().stdout("Foo\n").stderr("");
}
