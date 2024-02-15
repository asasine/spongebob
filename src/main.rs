// use arboard::Clipboard;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The space-separated words to modify.
    ///
    /// At least one word is required.
    #[arg(id = "WORD", required = true)]
    words: Vec<String>,

    /// Randomize the case of each letter, rather than alternating.
    #[arg(short, long)]
    random: bool,
}


fn main() {
    let args = Cli::parse();

    // alternate each letter's case of each word, skipping non-alphabetic characters
    let text = args.words.join(" ");
    let output = if args.random {
        spongebob::randomize(&text)
    } else {
        spongebob::alternate(&text)
    };

    println!("{}", output);

    // let mut clipboard = Clipboard::new().unwrap();
    // clipboard.set_text(args.words.join(" ")).unwrap();
}
