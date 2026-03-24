use clap::Parser;

#[derive(Parser)]
#[command(
    version,
    about = "A utility to a d d  s p a c e s to your text.",
    after_long_help = r#"You can copy to the clipboard with existing utilities:
  - Windows: `spongebob foo | clip`
  - macOS: `spongebob foo | pbcopy`
  - Linux (Wayland): `spongebob foo | wl-copy`
  - Linux (X11): `spongebob foo | xclip`
  - WSL: `spongebob foo | clip.exe`"#
)]
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
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        super::Cli::command().debug_assert();
    }
}
