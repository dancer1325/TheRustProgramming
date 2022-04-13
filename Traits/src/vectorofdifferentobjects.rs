trait Animal {
    fn name(&self) -> &'static str;     // &    Required to use ', to mark as static

    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}
struct Human {
    name: &'static str,
}

impl Animal for Human {
    fn name(&self) -> &'static str{
        self.name
    }
    fn talk(&self) {
        println!("Hello, my name is {}", self.name());
    }
}
struct Cat {
    name: &'static str,
}

impl Animal for Cat {
    fn name(&self) -> &'static str{
        self.name
    }
    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

// Ways to have several types in a vector
// 1) Using enum
enum Creature {
    Human(Human),           // NameOfTheEntry(FirstArgumentOfTheEnum)
    Cat(Cat)
}

pub fn demo() {
    let mut creatures = Vec::new();

    // Pushing directly  --> error[E0308]:      because you are adding different types to the vector
    // creatures.push(Human{name:"Alfredo"});
    // creatures.push(Cat {name:"Anubis"});

    // 1) enum
    creatures.push(Creature::Human(Human{name:"Alfredo"}));
    creatures.push(Creature::Cat(Cat{name:"Anubis"}));

    for c in creatures {
        match c {               // Required the matching, because c is the enum Creature and it hasn't got .talk()
            Creature::Human(h) => h.talk(),
            Creature::Cat(c) => c.talk(),
        }
    }

    // 2) Specify the type of vector, wrapping the trait
    // let mut creaturesTwo:Vec<dyn Animal> = Vec::new();          // error[E0277]
    // creaturesTwo.push(Human{name:"Alfredo"});
    // creaturesTwo.push(Cat {name:"Anubis"});

    let mut creaturesTwo:Vec<Box<dyn Animal>> = Vec::new();        // Box  it's size is known --> It can be an element in a vector
    creaturesTwo.push(Box::new(Human{name:"Alfredo"}));
    creaturesTwo.push(Box::new(Cat {name:"Anubis"}));

    for a in creaturesTwo.iter(){
        a.talk();                           // Unnecessary to match pattern, because you are iterating through animals
    }
}