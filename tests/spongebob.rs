use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};
use std::{
    io::Write,
    process::{Output, Stdio},
};

const LONG_TEST_TEXT: &str = r"
    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
    eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut
    enim ad minim veniam, quis nostrud exercitation ullamco laboris
    nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor
    in reprehenderit in voluptate velit esse cillum dolore eu fugiat
    nulla pariatur. Excepteur sint occaecat cupidatat non proident,
    sunt in culpa qui officia deserunt mollit anim id est laborum.
    ";

const LONG_TEST_TEXT_SPONGEBOB: &str = r"
    LoReM iPsUm DoLoR sIt AmEt, CoNsEcTeTuR aDiPiScInG eLiT, sEd Do
    EiUsMoD tEmPoR iNcIdIdUnT uT lAbOrE eT dOlOrE mAgNa AlIqUa. Ut
    EnIm Ad MiNiM vEnIaM, qUiS nOsTrUd ExErCiTaTiOn UlLamCo LaBoRiS
    nIsI uT aLiQuIp Ex Ea CoMmOdO cOnSeQuAt. DuIs AutE iRuRe DoLoR
    iN rEpReHeNdErIt In VoLuPtAtE vElIt EsSe CiLluM dOlOrE eU fUgIaT
    nUlLa PaRiAtUr. ExCePtEuR sInT oCcAeCaT cUpIdAtAt NoN pRoIdEnT,
    sUnT iN cUlPa QuI oFfIcIa DeSeRuNt MoLlIt AnIm Id EsT lAbOrUm.
    ";

/// Extension trait for [`Command`] to run a process with a given stdin.
trait SpawnExt {
    /// Spawn the process, write `input` to the stdin of the command, and return the output.
    fn run_with_stdin(&mut self, input: &str) -> Output;
}

impl SpawnExt for std::process::Command {
    fn run_with_stdin(&mut self, input: &str) -> Output {
        self.stdin(Stdio::piped()).stdout(Stdio::piped());
        let mut child = self.spawn().unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(input.as_bytes())
            .unwrap();

        child.wait_with_output().unwrap()
    }
}

#[test]
fn test_alternate_flag() {
    std::process::Command::cargo_bin("spongebob")
        .unwrap()
        .arg("--alternate")
        .arg("hello")
        .assert()
        .success()
        .stdout(predicates::str::contains("hElLo"));
}

#[test]
fn test_multiple_words_one_string() {
    std::process::Command::cargo_bin("spongebob")
        .unwrap()
        .arg("--alternate")
        .arg("Hello, world!")
        .assert()
        .success()
        .stdout(predicates::str::contains("HeLlO, wOrLd!"));
}

#[test]
fn test_multiple_words_multiple_strings() {
    std::process::Command::cargo_bin("spongebob")
        .unwrap()
        .arg("--alternate")
        .arg("Hello,")
        .arg("world!")
        .assert()
        .success()
        .stdout(predicates::str::contains("HeLlO, wOrLd!"));
}

#[test]
fn test_implicit_short_stdin() {
    std::process::Command::cargo_bin("spongebob")
        .unwrap()
        .arg("--alternate")
        .run_with_stdin("This is a test")
        .assert()
        .success()
        .stdout(predicates::str::contains("ThIs Is A tEsT"));
}

#[test]
fn test_explicit_short_stdin() {
    std::process::Command::cargo_bin("spongebob")
        .unwrap()
        .arg("--alternate")
        .arg("-")
        .run_with_stdin("This is a test")
        .assert()
        .success()
        .stdout(predicates::str::contains("ThIs Is A tEsT"));
}

#[test]
fn test_explicit_long_stdin() {
    std::process::Command::cargo_bin("spongebob")
        .unwrap()
        .arg("--alternate")
        .arg("-")
        .run_with_stdin(LONG_TEST_TEXT)
        .assert()
        .success()
        .stdout(predicates::str::contains(LONG_TEST_TEXT_SPONGEBOB));
}

#[test]
fn test_implicit_long_stdin() {
    std::process::Command::cargo_bin("spongebob")
        .unwrap()
        .arg("--alternate")
        .run_with_stdin(LONG_TEST_TEXT)
        .assert()
        .success()
        .stdout(predicates::str::contains(LONG_TEST_TEXT_SPONGEBOB));
}
