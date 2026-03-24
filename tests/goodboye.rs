use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};
use std::process::Command;

#[test]
fn test_alternate_flag() {
    Command::cargo_bin("goodboye")
        .unwrap()
        .arg("hello")
        .assert()
        .success()
        .stdout(predicates::str::contains("h e l l o "));
}

#[test]
fn test_multiple_words_one_string() {
    Command::cargo_bin("goodboye")
        .unwrap()
        .arg("Hello,    world!") // multiple spaces to test that they are preserved
        .assert()
        .success()
        .stdout(predicates::str::contains("H e l l o ,     w o r l d ! "));
}

#[test]
fn test_multiple_words_multiple_strings() {
    Command::cargo_bin("goodboye")
        .unwrap()
        .arg("Hello,")
        .arg("world!")
        .assert()
        .success()
        .stdout(predicates::str::contains("H e l l o ,  w o r l d ! "));
}
