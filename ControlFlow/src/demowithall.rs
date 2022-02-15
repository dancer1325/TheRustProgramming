use std::io::stdin;

// std    It's the Rust Standard Library
// io     Traits, helpers, and type definitions for core I/O functionality.
// stdin  Standard input stream of a process. Getting the commands introduced by console

// Play with control flows
// fn combinationlock() {  // By default it's private the function
pub fn combinationlock() {

    enum State {
        Locked,
        Failed,
        Unlocked
    }

    let code = String::from("1234");
    println!("code: {}", code);
    let mut state = State::Locked;
    let mut entry = String::new();
    println!("entry: {}", entry);

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                println!("input: {}", input);
                match stdin().read_line(&mut input) {  // If you type some characters and press "enter" --> You get a line
                    // Read the line and assign to "input" variable
                    // read_line return Result<usize>, which is an enum with 2 possible values; 1) Ok(T), 2) Err(T)
                    Ok(_) => {
                        // If code enters here, means that a character has been found
                        entry.push_str(&input.trim_end()); // Append the value got to entry, and remove the trailing whitespace
                    }
                    Err(_) => {
                        continue;
                    }
                }
                if entry == code {
                    state = State::Unlocked;
                    continue;
                }
                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("FAILED");
                entry.clear();
                state = State::Locked;
                continue;
            }
            State::Unlocked => {
                println!("UNLOCKED");
                return;
            }
        }
    }
}