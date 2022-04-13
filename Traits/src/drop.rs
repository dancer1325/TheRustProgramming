struct Creature {
    name: String,
}

impl Creature {                 // Extend the struct functionality without having to use a trait
    fn new(name: &str) -> Creature {
        println!("{} enters the game", name);
        Creature { name: name.into() }
    }
}

impl Drop for Creature {                        // Not necessary to implement it. It's invoked automatically by Rust
    fn drop(&mut self) {                        // Invoked when the code
        println!("{} is dead", self.name);
    }
}

pub fn demo() {
    let goblin = Creature::new("Jose");
    println!("Game proceeds");

    // goblin.drop();                          // It can't be invoked manually

    drop(goblin);                               // drop         As global function to manage the deletion
    println!("Game proceeds 2");                // After this message displayed, you can see that drop isn't executed again
    // println!("goblin.name is {}", goblin.name);          // Instance doesn't exist ever --> It throws an error

    // Drop is launched at the end of the scope
    let mut clever: Creature;
    {
        println!("Start of the scope");
        let goblinTwo = Creature::new("Jose Enrique");
        clever = goblinTwo;
        println!("End of the scope");                   //This message is displayed previously to the one which belongs to the deletion of the instance goblinTwo
    }
}