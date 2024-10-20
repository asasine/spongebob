
use clap::Parser;

#[derive(Parser)]
#[command(version, about = "A utility to a d d  s p a c e s to your text.")]
struct Cli {
    /// The space-separated words to modify.
    ///
    /// At least one word is required.
    #[arg(id = "WORD", required = true)]
    words: Vec<String>,
}

fn main() {
    human_panic::setup_panic!();
    let args = Cli::parse();
    let text = args.words.join(" ");
    let output = spongebob::spaceify(text.as_str());

    println!("{}", output);
}


#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        super::Cli::command().debug_assert();
    }

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
}
