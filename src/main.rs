// use arboard::Clipboard;
use clap::Parser;
use spongebob::alternate;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The space-separated words to modify.
    ///
    /// At least one word is required.
    #[arg(id = "WORD", required = true)]
    words: Vec<String>,
}


fn main() {
    let args = Cli::parse();

    // alternate each letter's case of each word, skipping non-alphabetic characters
    let alternated = alternate(&args.words.join(" "));

    println!("{}", alternated);

    // let mut clipboard = Clipboard::new().unwrap();
    // clipboard.set_text(args.words.join(" ")).unwrap();
}
