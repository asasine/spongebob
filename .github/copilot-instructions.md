# Spongebob CLI Tool
Spongebob is a Rust CLI application that converts text to "spongebob case" (alternating/random case) for the mocking SpongeBob meme. The project includes two binaries: `spongebob` (main tool) and `goodboye` (adds spaces between characters).

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively
- Bootstrap, build, and test the repository:
  - Rust toolchain is pre-installed (rustc 1.89.0, cargo 1.89.0)
  - `cargo build --verbose` -- builds debug version, takes ~25 seconds. NEVER CANCEL. Set timeout to 60+ seconds.
  - `cargo build --release --verbose` -- builds optimized version, takes ~18 seconds. NEVER CANCEL. Set timeout to 60+ seconds.
  - `cargo test --verbose` -- runs full test suite, takes ~10 seconds. NEVER CANCEL. Set timeout to 30+ seconds.
  - `cargo doc --no-deps` -- generates documentation, takes ~2 seconds.
- Format and lint code:
  - `cargo fmt` -- formats code (required before committing)
  - `cargo clippy -- -D warnings` -- lints code and fails on warnings
- Install locally:
  - `cargo install --path .` -- installs both `spongebob` and `goodboye` binaries, takes ~25 seconds. NEVER CANCEL. Set timeout to 60+ seconds.

## Validation
- Always manually validate any new code by testing actual CLI functionality.
- ALWAYS run through at least one complete end-to-end scenario after making changes:
  - Test `./target/debug/spongebob --alternate "hello world"` should output `hElLo WoRlD`
  - Test `./target/debug/spongebob "hello world"` should output randomized case like `HeLlo WoRld`
  - Test `echo "test input" | ./target/debug/spongebob --alternate` should output `tEsT iNpUt`
  - Test `./target/debug/goodboye "hello world"` should output `h e l l o  w o r l d`
- Always run `cargo fmt` and `cargo clippy -- -D warnings` before you are done or the CI (.github/workflows/rust.yaml) will fail.

## Common Tasks
The following are outputs from frequently run commands. Reference them instead of viewing, searching, or running bash commands to save time.

### Repository Root
```
ls -a /home/runner/work/spongebob/spongebob
.git .github .gitignore CHANGELOG.md CONTRIBUTING.md Cargo.lock Cargo.toml LICENSE README.md src
```

### Source Structure
```
src/
├── lib.rs          # Library functions: alternate(), randomize(), spaceify()
├── main.rs         # Main spongebob CLI binary
└── bin/
    └── goodboye.rs # Secondary CLI binary for spacing text
```

### Key Files
- `Cargo.toml` -- Project configuration and dependencies
- `src/lib.rs` -- Core library with text transformation functions
- `src/main.rs` -- Main CLI application with clap argument parsing
- `src/bin/goodboye.rs` -- Secondary CLI for adding spaces
- `.github/workflows/rust.yaml` -- CI workflow (build and test)
- `README.md` -- Usage examples and installation instructions

### Dependencies (from Cargo.toml)
```toml
[dependencies]
anstyle = "1.0.7"
clap = { version = "4.5", features = ["derive", "unicode"] }
human-panic = "2.0"
rand = "0.8"

[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.1"
```

### Test Coverage
- Unit tests in `src/lib.rs` for core functions (alternate, randomize, spaceify)
- Integration tests in `src/main.rs` using assert_cmd for CLI testing
- Integration tests in `src/bin/goodboye.rs` for goodboye CLI
- Doc tests embedded in function documentation
- All tests run in ~10 seconds with `cargo test`

### Build Artifacts
- Debug binaries: `target/debug/spongebob`, `target/debug/goodboye`
- Release binaries: `target/release/spongebob`, `target/release/goodboye`
- Documentation: `target/doc/spongebob/index.html`

## Library API
The library exports three main functions:
- `alternate(text: &str) -> String` -- Alternates case of each letter
- `randomize(text: &str) -> String` -- Randomizes case of each letter  
- `spaceify(text: &str) -> String` -- Inserts spaces between characters

## CLI Usage Examples
```bash
# Basic usage (randomized case)
spongebob hello world
# Output: heLlO WoRld (varies due to randomization)

# Alternate case
spongebob --alternate hello world  
# Output: hElLo WoRlD

# Stdin input
echo "hello world" | spongebob --alternate
# Output: hElLo WoRlD

# Goodboye spacing
goodboye "hello world"
# Output: h e l l o  w o r l d
```

## Critical Build Information
- **NEVER CANCEL** builds or tests - they may take up to 25 seconds for builds and 10 seconds for tests
- Always set timeouts to 60+ seconds for build commands and 30+ seconds for test commands
- First builds download dependencies and take longer; subsequent builds are faster
- Release builds are faster than debug builds (~18s vs ~25s)
- No special environment setup required beyond Rust toolchain
- No external dependencies or databases required
- Builds work on all platforms (Linux, macOS, Windows)

## Common Workflows
1. **Making changes to library functions**: Edit `src/lib.rs`, run `cargo test`, then test CLI manually
2. **Making changes to CLI**: Edit `src/main.rs` or `src/bin/goodboye.rs`, run `cargo build`, test CLI manually
3. **Adding new features**: Add tests first, implement feature, run full validation
4. **Before committing**: Always run `cargo fmt && cargo clippy -- -D warnings && cargo test`