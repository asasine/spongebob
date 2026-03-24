use assert_cmd::Command;

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

#[test]
fn test_alternate_flag() {
    Command::cargo_bin("spongebob")
        .unwrap()
        .arg("--alternate")
        .arg("hello")
        .assert()
        .success()
        .stdout(predicates::str::contains("hElLo"));
}

#[test]
fn test_multiple_words_one_string() {
    Command::cargo_bin("spongebob")
        .unwrap()
        .arg("--alternate")
        .arg("Hello, world!")
        .assert()
        .success()
        .stdout(predicates::str::contains("HeLlO, wOrLd!"));
}

#[test]
fn test_multiple_words_multiple_strings() {
    Command::cargo_bin("spongebob")
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
    Command::cargo_bin("spongebob")
        .unwrap()
        .arg("--alternate")
        .write_stdin("This is a test")
        .assert()
        .success()
        .stdout(predicates::str::contains("ThIs Is A tEsT"));
}

#[test]
fn test_explicit_short_stdin() {
    Command::cargo_bin("spongebob")
        .unwrap()
        .arg("--alternate")
        .arg("-")
        .write_stdin("This is a test")
        .assert()
        .success()
        .stdout(predicates::str::contains("ThIs Is A tEsT"));
}

#[test]
fn test_explicit_long_stdin() {
    Command::cargo_bin("spongebob")
        .unwrap()
        .arg("--alternate")
        .arg("-")
        .write_stdin(LONG_TEST_TEXT)
        .assert()
        .success()
        .stdout(predicates::str::contains(LONG_TEST_TEXT_SPONGEBOB));
}

#[test]
fn test_implicit_long_stdin() {
    Command::cargo_bin("spongebob")
        .unwrap()
        .arg("--alternate")
        .write_stdin(LONG_TEST_TEXT)
        .assert()
        .success()
        .stdout(predicates::str::contains(LONG_TEST_TEXT_SPONGEBOB));
}
