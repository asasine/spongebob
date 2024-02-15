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
        clipboard.set_text(output).unwrap();
    }
}
