use std::io::Read;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The space-separated words to modify.
    ///
    /// If no words were provided, the words may be given from stdin. Input from stdin can be
    /// explicity requested by passing "-" as the first and only word.
    #[arg(id = "WORD", required = false)]
    words: Vec<String>,

    /// Alternate the case of each letter, rather than randomizing.
    #[arg(short, long)]
    alternate: bool,
}

fn main() {
    human_panic::setup_panic!();
    let args = Cli::parse();

    // using "-" as an argument for a file usually signals that the user wants the input to be read
    // from the stdin. If no words were given, we can just assume input from stdin.
    let use_stdin = args.words.is_empty() || (args.words.len() == 1 && args.words[0] == "-");
    let mut output: String;

    if use_stdin {
        let mut stdin = std::io::stdin();
        let mut buf = [0; 64];
        let mut repr: String;
        let mut stream: String;
        output = String::new();
        loop {
            let len = match stdin.read(&mut buf) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Could not read from stdin: {e}");
                    std::process::exit(1)
                }
            };

            if len == 0 {
                break; // end of file reached
            }

            match std::str::from_utf8(&buf[0..len]) {
                Ok(s) => repr = s.to_string(),
                Err(e) => {
                    eprintln!("The data provided via the stdin is not text, but this application only accepts text: {e}");
                    std::process::exit(1)
                }
            }

            stream = if args.alternate {
                spongebob::alternate(&repr)
            } else {
                spongebob::randomize(&repr)
            };

            print!("{}", stream);
            output += &stream;
            stream.clear();
        }
    } else {
        let words = args.words.join(" ");
        output = if args.alternate {
            spongebob::alternate(&words)
        } else {
            spongebob::randomize(&words)
        };

        println!("{}", output);
    }
}

#[cfg(test)]
mod tests {
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
    fn verify_cli() {
        use clap::CommandFactory;
        super::Cli::command().debug_assert();
    }

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
}
