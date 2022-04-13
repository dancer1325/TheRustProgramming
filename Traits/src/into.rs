#![allow(unused_mut)]
#![allow(unused_variables)]
struct Person {
    name: String
}
impl Person {                                   // Extend the struct functionality without having to use a trait
    fn new(name: &str) -> Person {              // str
        Person { name: name.to_string()}        // to_string        Convert to a String, not str
    }
    fn new2<S: Into<String>>(name: S) -> Person {                          // Generic is a type which can be converted into String
        Person { name: name.into()}
    }
    fn new3<S>(name: S) -> Person                                          // Same as previous one, but specifying the generic by where
        where S: Into<String>
    {                          // Generic is a type which can be converted into String
        Person { name: name.into()}
    }
}

pub fn demo(){
    let alfredo = Person::new("Alfredo");

    let name: String = "Alfredo".to_string();
    let alfredo2 = Person::new(name.as_ref());          //as_ref    Required to use, because the argument is str

    let alfredo3 = Person::new2(name);                  // new2     Not necessary to convert previously the argument

    // new3     Not necessary to convert previously the argument
    // let alfredo4 = Person::new3(name);                  // error[E0382] TODO: Check if it's possible to fix it
    let nameTwo: String = "Alfredo".to_string();
    let alfredo4 = Person::new3(nameTwo);
}