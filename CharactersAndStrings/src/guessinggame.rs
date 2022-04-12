use rand::Rng;
use std::io::stdin;

pub fn demo() {
    // let number = rand::thread_rng().gen_range(1, 101);          // 101 is excluded       For 0.5 rand versions
    let number = rand::thread_rng().gen_range(1..101);          // 101 is excluded

    loop {
        // print!("Enter your guess: ");       // If we use it, it's displayed after read_line method
        println!("Enter your guess: ");       // Not to break the line

        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                // trim_end     Delete the tailing space, because it takes all the whole line
                // parsing      It can be done: A) Declaring the variable, B) At the end in the .parse method
                let parsed = buffer.trim_end().parse::<i64>();
                // let parsed:i64 = buffer.trim_end();              //TODO: How to force the type, declaring the variable?
                 match parsed {
                     Ok(guess) => {
                         if guess < 1 || guess > 100 {
                             println!("Your guess {} is out of range", guess);
                         } else if guess < number {
                             println!("Your guess {} is too low", guess);
                         } else if guess > number {
                             println!("Your guess {} is too high", guess);
                         } else {
                             println!("Correct!!! {} was the desired number ", guess);
                             break;
                         }
                     }
                     Err(e) => {
                         println!("Couldn't read your input {}. Try again", e);
                     }
                 }
            }
            Err(error) => {
                println!("error: {}", error);
                continue;               // Jump to the next iteration of the loop
            }
        }
    }
}