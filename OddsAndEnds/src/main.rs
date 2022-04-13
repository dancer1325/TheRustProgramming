extern crate rand;                  // export crates
extern crate OwnCrate;

use rand::Rng;                      // Use a module
use OwnCrate::greetings;
use OwnCrate::greetings::french;    // Use an internal module, to reduce the boilerplate code

fn main() {
    println!("English: {}, {}", greetings::english::hello(), greetings::english::goodbye());
    println!("French: {}, {}", french::hello(), french::goodbye());
}
