#![allow(dead_code)]
#![allow(unused_variables)]

use std::rc::Rc;

struct Person {
    // name: String,
    name: Rc<String>,           // Wrap it with Rc, to keep track the number of references
}

impl Person {
    // fn new(name: String) -> Person {
    fn new(name: Rc<String>) -> Person {        // Wrap it with Rc, to keep track the number of references
     Person{ name: name}
    }
    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}
pub fn demo(){
    // Without using Rc
    // let name = "Alfredo".to_string();
    // let person = Person::new(name);             // name is moved in the local scope

    // person.greet();
    // println!("Name of the person is {}", name);             // error[E0382]


    // Using Rc
    let name = Rc::new("Alfredo".to_string());
    println!("Name {} has got {} strong pointers", name, Rc::strong_count(&name));
    let person = Person::new(name.clone());             // .clone()     Required to indicate which it's another reference
    person.greet();
    println!("Name of the person is {}", name);
    println!("Name {} has got {} strong pointers", name, Rc::strong_count(&name));
    {
        let person = Person::new(name.clone());             // This new reference just lives into this block of code
        println!("Name {} has got {} strong pointers", name, Rc::strong_count(&name));
    }
    println!("Name {} has got {} strong pointers", name, Rc::strong_count(&name));      // Reference created into the block of code disappears
}