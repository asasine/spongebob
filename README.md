# Spongebob
A utility to convert text to spongebob case a.k.a tHe MoCkInG sPoNgEbOb MeMe.

[![Crates.io Version](https://img.shields.io/crates/v/spongebob?logo=rust)](https://crates.io/crates/spongebob)
[![Crates.io Documentation](https://docs.rs/spongebob/badge.svg)](https://docs.rs/spongebob)
[![CI](https://img.shields.io/github/actions/workflow/status/asasine/spongebob/rust.yaml?branch=main&logo=github&label=CI)](https://github.com/asasine/spongebob/actions/workflows/rust.yaml?query=branch%3Amain)
[![Crates.io Downloads](https://img.shields.io/crates/d/spongebob)](https://crates.io/crates/spongebob)


# Examples
The command randomizes the words by default:
```bash
spongebob hello world
# heLlO WoRld
```

You can also alternate with `--alternate`:
```bash
spongebob --alternate hello world
# hElLo WoRlD
```

You can copy to the clipboard with existing utilities:

- Windows: `spongebob foo | clip`
- macOS: `spongebob foo | pbcopy`
- Linux (Wayland): `spongebob foo | wl-copy`
- Linux (X11): `spongebob foo | xclip`
- WSL: `spongebob foo | clip.exe`

# Installation
```bash
cargo install spongebob
```

# Library
Library functions are available to use in your own projects. See the [documentation](https://docs.rs/spongebob) for more information.
