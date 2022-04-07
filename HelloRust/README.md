# Hello Rust
* Section 1 of "The Rust Programming Language"

# How to compile?
* `rustc NameOfTheFile.rs`
  * Generate the 'NameOfTheFile' executable
    * `.NameOfTheFile`
      * Launch the executable

# Notes
* cargo
  * Another tool included into Rust installation
  * Allows
    * avoid using `rustc` command
    * using them as dependency manager
  * `cargo build`
    * Compile
      * 'target' folder
    * 'cargo.lock'
  * `cargo run`
    * Compile
      * 'target' folder
    * 'cargo.lock'
    * Execute
  * `cargo new NameOfTheProject`
    * Create all the structure of the project
    * `--bin`
      * For a binary
      * Uses
        * create an '.exe'
        * not for a library creation