# Own Crate
* Part of section 12 of "The Rust Programming Language" course

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
            * Download the dependencies indicated in 'cargo.toml'
        * 'cargo.lock'
        * Execute

# Notes
* Rust understands each module tree as folder structure
  * You can create that structure, and it will go to pick each file
    * Example:
      * 'english.rs' file created under greetings folder
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
    * `cargo build`
        * Download all the external dependencies from the crates registry
        * Build all
    * `cargo test`
      * Run the tests created
* `tree`
  * Check all the tree of files
  * Specific command of your OS
    * Valid in
      * [Ubuntu]
      * [Mac]
* `rustdoc NameOfFileWithDocumentation.rs`
  * It takes the documentation indicated in the file
  * Create 'doc' folder
  * 
