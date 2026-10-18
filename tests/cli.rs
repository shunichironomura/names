#![cfg(feature = "application")]

use std::process::{Command, Output};

fn names(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_names"))
        .args(args)
        .output()
        .expect("names should start")
}

#[test]
fn default_generates_one_name() {
    let output = names(&[]);
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.lines().count(), 1);
    assert!(!stdout.trim().is_empty());
}

#[test]
fn numbered_names_respect_amount_and_both_flag_forms() {
    for flag in ["-n", "--number"] {
        let output = names(&[flag, "5"]);
        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert_eq!(stdout.lines().count(), 5);
        assert!(stdout.lines().all(|name| {
            let suffix = name.rsplit('-').next().unwrap();
            suffix.len() == 4 && suffix.parse::<u16>().is_ok_and(|n| (1..10000).contains(&n))
        }));
    }
}

#[test]
fn zero_generates_no_names() {
    let output = names(&["0"]);
    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}

#[test]
fn help_and_version_are_available() {
    let help = names(&["--help"]);
    assert!(help.status.success());
    let help = String::from_utf8(help.stdout).unwrap();
    assert!(help.contains("[AMOUNT]"));
    assert!(help.contains("--number"));

    let version = names(&["--version"]);
    assert!(version.status.success());
    assert_eq!(
        String::from_utf8(version.stdout).unwrap().trim(),
        concat!("names ", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn invalid_arguments_fail() {
    for args in [&["not-a-number"][..], &["--unknown"][..]] {
        let output = names(args);
        assert!(!output.status.success());
        assert!(!output.stderr.is_empty());
    }
}
