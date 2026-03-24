use std::io::Read;

use clap::Parser;

#[derive(Parser)]
#[command(
    version,
    about = "A utility to convert text to spongebob case a.k.a tHe MoCkInG sPoNgEbOb MeMe.",
    long_about = None,
    after_long_help = spongebob::clipboard_help!("spongebob"),
)]
struct Cli {
    /// The space-separated words to modify, or "-" to read from stdin.
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
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        super::Cli::command().debug_assert();
    }
}
