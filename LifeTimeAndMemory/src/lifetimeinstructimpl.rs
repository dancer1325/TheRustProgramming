// struct Person {
struct Person<'a> {                 // struct's lifetime == name's lifetime
    // name: &str                   // error[E0106]:            Lifetime elision doesn't work here
    name: &'a str
}
// impl Person {                    // error[E0726]:    Necessary to indicate the lifetime in the impl
impl<'b> Person<'b> {               // b    Not necessary to mark as a, just necessary to mark that the impl's lifetime == struct's lifetime
    fn talk(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

pub fn demo() {
    let person = Person { name: "Alfredo Toledano" };
    person.talk();
}