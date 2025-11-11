// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

#[cfg(feature = "cli")]
use {assert_cmd::cargo::cargo_bin_cmd, predicates::prelude::*};

#[cfg(feature = "cli")]
#[test]
fn main() {
    let mut cmd = cargo_bin_cmd!(env!("CARGO_PKG_NAME"));
    cmd.args(["foo"])
        .assert()
        .success()
        .stdout("Foo\n")
        .stderr("");
}

#[cfg(feature = "cli")]
#[test]
fn main_stdin() {
    let mut cmd = cargo_bin_cmd!(env!("CARGO_PKG_NAME"));
    cmd.write_stdin("foo")
        .assert()
        .success()
        .stdout("Foo\n")
        .stderr("");
}

#[cfg(feature = "cli")]
#[test]
fn main_help() {
    let mut cmd = cargo_bin_cmd!(env!("CARGO_PKG_NAME"));
    cmd.args(["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage: "))
        .stderr("");
}

#[cfg(feature = "cli")]
#[test]
fn main_overrides() {
    let mut cmd = cargo_bin_cmd!(env!("CARGO_PKG_NAME"));
    cmd.args(["-l", "en", "-O", "fOO", "--", "foo bar"])
        .assert()
        .success()
        .stdout("fOO Bar\n")
        .stderr("");
}

#[cfg(feature = "cli")]
#[test]
fn main_lang() {
    let mut en_cmd = cargo_bin_cmd!(env!("CARGO_PKG_NAME"));
    en_cmd
        .args(["-l", "en", "ide"])
        .assert()
        .success()
        .stdout("Ide\n")
        .stderr("");
    let mut tr_cmd = cargo_bin_cmd!(env!("CARGO_PKG_NAME"));
    tr_cmd
        .args(["-l", "tr", "ilk"])
        .assert()
        .success()
        .stdout("İlk\n")
        .stderr("");
}
