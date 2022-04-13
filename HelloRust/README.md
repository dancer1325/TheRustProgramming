# Hello Rust
* Section 1 of "The Rust Programming Language"

# How to compile?
* Alternatives
  * `rustc NameOfTheFile.rs`
    * Generate the 'NameOfTheFile' executable
      * `.NameOfTheFile`
        * Launch the executable
  * `cargo build`
    * Compile
      * 'target' folder
    * 'cargo.lock'

# How to execute?
* Alternatives
  * `./NameOfTheFileExecutable`
    * After using rustc command
    * Example
      * `./main`
  * `cargo run`
    * Compile
      * 'target' folder
    * 'cargo.lock'
    * Execute

# Notes
* cargo
  * Another tool included into Rust installation
  * Allows
    * avoid using `rustc` command
    * using them as dependency manager
  * `cargo new NameOfTheProject`
    * Create all the structure of the project
    * `--bin`
      * For a binary
      * Uses
        * create an '.exe'
        * not for a library creation