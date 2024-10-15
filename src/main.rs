use std::io::Read;

use arboard::Clipboard;
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

    /// Copy the modified text to the clipboard.
    #[arg(short, long, default_value = "false")]
    no_copy: bool,
}

fn main() {
    human_panic::setup_panic!();
    let args = Cli::parse();

    // using "-" as an argument for a file usually signals that the user wants the input to be read
    // from the stdin. If no words were given, we can just assume input from stdin.
    let text: String = if !(args.words.is_empty() || args.words.len() == 1 && args.words[0] == "-")
    {
        args.words.join(" ")
    } else {
        let mut buf = String::new();
        let mut stdin = std::io::stdin();
        let _textlen = stdin.read_to_string(&mut buf);
        buf
    };

    let output = if args.alternate {
        spongebob::alternate(&text)
    } else {
        spongebob::randomize(&text)
    };

    println!("{}", output);

    if !args.no_copy {
        match Clipboard::new() {
            Ok(mut clipboard) => {
                clipboard
                    .set_text(output)
                    .expect("Failed to copy to clipboard.");

                eprintln!("Copied to clipboard.");
            }
            Err(e) => {
                if cfg!(target_os = "linux") {
                    let yellow = anstyle::AnsiColor::Yellow.on_default();
                    eprintln!("\n{yellow}Looks like you're experiencing https://github.com/asasine/spongebob/issues/8. Clipboard support on Linux is currently unreliable. In the meantime, try piping to wl-copy from the wl-clipboard package or copying from your terminal above.{yellow:#}");
                    eprintln!("\nIf you can, please add a 👍 or comment to https://github.com/asasine/spongebob/issues/8 so I can prioritize fixing this.\n");
                }

                panic!("Failed to access clipboard: {}", e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use arboard::Clipboard;
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
    EnIm Ad MiNiM vEnIaM, qUiS nOsTrUd ExErCiTaTiOn UlLaMcO lAbOrIs
    NiSi Ut AlIqUiP eX eA cOmMoDo CoNsEqUaT. dUiS aUtE iRuRe DoLoR
    iN rEpReHeNdErIt In VoLuPtAtE vElIt EsSe CiLlUm DoLoRe Eu FuGiAt
    NuLlA pArIaTuR. eXcEpTeUr SiNt OcCaEcAt CuPiDaTaT nOn PrOiDeNt,
    SuNt In CuLpA qUi OfFiCiA dEsErUnT mOlLiT aNiM iD eSt LaBoRuM.
    ";

    #[test]
    fn test_alternate_flag() {
        Command::cargo_bin("spongebob")
            .unwrap()
            .arg("--alternate")
            .arg("--no-copy")
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
            .arg("--no-copy")
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
            .arg("--no-copy")
            .arg("Hello,")
            .arg("world!")
            .assert()
            .success()
            .stdout(predicates::str::contains("HeLlO, wOrLd!"));
    }

    #[ignore = "Clipboard access is not reliable in CI"]
    #[test]
    fn test_copies_to_clipboard() {
        let mut clipboard = Clipboard::new().unwrap();
        clipboard.clear().unwrap();
        assert!(clipboard.get_text().is_err());

        Command::cargo_bin("spongebob")
            .unwrap()
            .arg("--alternate")
            .arg("hello")
            .assert()
            .success()
            .stdout(predicates::str::contains("clipboard"));

        assert_eq!(clipboard.get_text().unwrap(), "hElLo");
    }

    #[ignore = "Clipboard access is not reliable in CI"]
    #[test]
    fn test_no_copy() {
        let mut clipboard = Clipboard::new().unwrap();
        clipboard.set_text("unimportant").unwrap();
        clipboard.clear().unwrap();
        assert!(clipboard.get_text().is_err(), "Clipboard should be empty");

        Command::cargo_bin("spongebob")
            .unwrap()
            .arg("--no-copy")
            .arg("hello")
            .assert()
            .success();

        assert!(
            clipboard.get_text().is_err(),
            "Clipboard should still be empty"
        );
    }

    #[test]
    fn test_implicit_short_stdin() {
        Command::cargo_bin("spongebob")
            .unwrap()
            .arg("--no-copy")
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
            .arg("--no-copy")
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
            .arg("--no-copy")
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
            .arg("--no-copy")
            .arg("--alternate")
            .write_stdin(LONG_TEST_TEXT)
            .assert()
            .success()
            .stdout(predicates::str::contains(LONG_TEST_TEXT_SPONGEBOB));
    }
}
