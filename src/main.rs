use arboard::Clipboard;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The space-separated words to modify.
    ///
    /// At least one word is required.
    #[arg(id = "WORD", required = true)]
    words: Vec<String>,

    /// Alternate the case of each letter, rather than randomizing.
    #[arg(short, long)]
    alternate: bool,

    /// Copy the modified text to the clipboard.
    #[arg(short, long, default_value = "false")]
    no_copy: bool,
}

fn main() {
    let args = Cli::parse();
    let text = args.words.join(" ");
    let output = if args.alternate {
        spongebob::alternate(&text)
    } else {
        spongebob::randomize(&text)
    };

    println!("{}", output);

    if !args.no_copy {
        let mut clipboard = Clipboard::new().expect("Failed to access clipboard");
        clipboard
            .set_text(output)
            .expect("Failed to copy to clipboard.");

        println!("Copied to clipboard.");
    }
}

#[cfg(test)]
mod tests {
    use arboard::Clipboard;
    use assert_cmd::Command;

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
}
