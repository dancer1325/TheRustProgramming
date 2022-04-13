#![allow(dead_code)]
#![allow(unused_variables)]

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

struct Person {
    name: Arc<String>,           // Wrap it with Arc, to keep track the number of references even in multiple threads
    // state: Arc<String>           // It doesn't protect from concurrent access, just reference counted
    state: Arc<Mutex<String>>
}

impl Person {
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person {
        Person{ name: name, state: state}
    }
    fn greet(&self) {
        // println!("Hi, my name is {} and my state is {}", self.name, self.state.as_str());                                // error[E0599]
        // println!("Hi, my name is {} and my state is {}", self.name, self.state..lock().unwrap().as_str()());            // error[E0277]
        // TODO: How to display self.state value?

        //error[E0596]
        // self.state.clear();                                 // Clear the value
        // self.state.push_str("excited");                     // Muting the value of state

        let mut state = self.state.lock().unwrap();
        state.clear();
        state.push_str("excited");
        println!("Hi, my name is {} and my state is {}", self.name, state.as_str());
    }
}
pub fn demo(){
    let name_two = Arc::new("Alfredo".to_string());
    let state = Arc::new(Mutex::new("bored".to_string()));
    let person_two = Person::new(name_two.clone(), state.clone());             // .clone()     Required to indicate which it's another reference

    let t = thread::spawn(move || {                     // spawn's argument is a moving closure
        person_two.greet();                             // Message is displayed after next line, because it's executed in another thread, and takes more time
    });

    println!("Name is {} and the state is {}", name_two, state.lock().unwrap().as_str());

    t.join().unwrap();                                  // .join()      Wait for the thread is completed
}