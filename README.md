# Spongebob
A utility to convert text to spongebob case a.k.a tHe MoCkInG sPoNgEbOb MeMe.

Cross-platform clipboard support for Windows, macOS, and Linux included.

# Examples
The command randomizes the words by default and copies them to the clipboard:
```bash
spongebob hello world
# heLlO WoRld
```

You can also alternate with `--alternate`:
```bash
spongebob --alternate hello world
# hElLo WoRlD
```

You can disable the clipboard with `--no-copy`:
```bash
spongebob --no-copy hello world
```

# Installation
```bash
cargo install spongebob
```

# Library
Library functions are available to use in your own projects. See the [documentation](https://docs.rs/spongebob) for more information.
