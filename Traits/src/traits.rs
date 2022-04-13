trait Animal {
    fn name(&self) -> &'static str;         // self     Points to the class in which it's used

    // Method with default implementation --> Not necessary to implement when it's used
    fn talk(&self) {
      println!("{} cannot talk", self.name());
    }

    // Static function      self isn't passed as argument
    fn create(name: &'static str) -> Self;          // Important!!! It's the type of current object. Different to self
}

// Define a struct implementing the trait
struct Human {
    name: &'static str
}
impl Animal for Human {
    // It can be used as alternative to create an instance of the struct
    fn create(name: &'static str) -> Human {
        // Human{name: name}
        Human{ name }           // Shorthand passing the argument
    }
    fn name(&self) -> &'static str {
        self.name               // ;        Not used, because it's the returned object
    }
    // Not necessary to implement talk method, since it has got a default logic
    fn talk(&self) {
        println!("{} says hello", {self.name});
    }
}

// Define a struct implementing the trait
struct Cat {
    name: &'static str
}
impl Animal for Cat {
    // It can be used as alternative to create an instance of the struct
    fn create(name: &'static str) -> Cat {
        Cat{ name }
    }
    fn name(&self) -> &'static str {
        self.name               // ;        Not used, because it's the returned object
    }
    // Not necessary to implement talk method, since it has got a default logic
    fn talk(&self) {
        println!("{} says meow", {self.name});
    }
}

// Create a trait for Generic. Example: Vector
trait Summable<T> {
    fn sum(&self) -> T;
}
impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result:i32 =0;
        for x in self {
            result += x;
        }
        result
    }
}

pub fn demo(){
    let alfredo = Human{name:"Alfredo"};
    alfredo.talk();

    // Create an instance based on the static function
    let alfredo2 = Human::create("Alfredo");
    alfredo2.talk();

    // Create an instance based on the static function of the trait
    // let alfredo3 = Animal::create("Alfredo");                    // It doesn't work because Self in trait isn't specified
    let alfredo3:Human = Animal::create("Alfredo");           // Specify the type of the instance, and infer it by the compiler
    alfredo3.talk();

    let anubis = Cat{name:"Anubis"};
    anubis.talk();

    // Create an instance based on the static function
    let anubis2 = Cat::create("Anubis");
    anubis2.talk();

    // Extending properties of vector
    let a = vec![1, 2, 3];
    println!("sum = {}", a.sum());
}