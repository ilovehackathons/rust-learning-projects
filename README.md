# The Rust Programming Language Book Project
As I go through the book, I'm putting all my code here.
## Commands
### rustc/other
- `rustc <filename>.rs` to compile a file
- `./<filename>` to run it
- `rustfmt` to format the code (need to RTFM first)
- `curl --proto "=https" --tlsv1.3 https://sh.rustup.rs -sSf | sh` to install Rust
- `xcode-select --install` to install a linker on macOS
- `rustc --version` to check the version number
- `echo $PATH` to check if Rust is in your execution path
- `rustup update` to check for updates and install them
- `rustup self uninstall` to remove Rust from your system
- `git status --ignored -uall` (nothing to do with Rust, but still interesting)
### Cargo
- `cargo new` to create a new project
- `cargo build` to compile the source code
- `cargo build --release` to compile into an optimized binary
- `cargo run` to compile the project (if not already compiled) and run it
- `cargo check` to check if the project compiles
- `cargo --version` to check the version number
## Directory structure
- `Cargo.toml` contains project metadata, including dependencies
- `src/` is where you put your source code
- `src/main.rs` contains the 'main' program
- `target/` is where the binaries go
- `target/debug` is for the dev executables
- `target/release` is for the production executables
- `Cargo.lock` tracks the exact dependency versions
## Syntax
- In `fn main() {}`, `fn` declares a function, `main` is its name, `()` is an empty parameter list and `{}` is an empty body
- `main` is the first function that gets executed in a Rust program (it can call other functions)
- In `println!("Hello, world!");`, `!` indicates that we're calling a macro (not a function), `println` (print line) is the name of the macro, `(` and `)` start and end the argument list, respectively, `"` and `"` signify the start and end of a string literal, respectively, and `Hello, world` is the actual text that will be printed to the console.
## Code style
- Use 4 spaces for each level of indentation (no tabs)
- Keep the opening curly bracket (`{`) on the same line as the function declaration
