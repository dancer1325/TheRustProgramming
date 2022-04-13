#![allow(dead_code)]
#![allow(unused_variables)]

use std::rc::Rc;
use std::sync::Arc;
use std::thread;

struct Person {
    // name: String,
    // name: Rc<String>,           // Wrap it with Rc, to keep track the number of references
    name: Arc<String>,           // Wrap it with Arc, to keep track the number of references even in multiple threads
}

impl Person {
    // fn new(name: String) -> Person {
    // fn new(name: Rc<String>) -> Person {        // Wrap it with Rc, to keep track the number of references
    fn new(name: Arc<String>) -> Person {        // Wrap it with Arc, to keep track the number of references even in multiple threads
        Person{ name: name}
    }
    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}
pub fn demo(){
    // 1) Using Rc   -- Code from referencecounted.rsd
    // let name = Rc::new("Alfredo".to_string());
    // let person = Person::new(name.clone());             // .clone()     Required to indicate which it's another reference

    // Separated thread to invoke .greet()
    // error[E0277]
    // let t = thread::spawn(move || {                     // spawn's argument is a moving closure
    //     person.greet();
    // });

    // println!("Name is {}", name);
    //
    // t.join().unwrap();                                  // .join()      Wait for the thread is completed

    // 2) Using Arc
    let name_two = Arc::new("Alfredo".to_string());
    let person_two = Person::new(name_two.clone());             // .clone()     Required to indicate which it's another reference

    let t = thread::spawn(move || {                     // spawn's argument is a moving closure
        person_two.greet();
    });

    println!("Name is {}", name_two);

    t.join().unwrap();                                  // .join()      Wait for the thread is completed
}