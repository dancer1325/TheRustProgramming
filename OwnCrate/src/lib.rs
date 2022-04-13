// By default all the modules and functions are private
pub mod greetings {
    // Parent module
    pub mod english;                // Plugin complains, but it works fine
        // Child module

    pub mod french {
        // Child module
        pub fn hello() -> String {
            "bonjour".to_string()
        }

        pub fn goodbye() -> String {
            "au revoir".to_string()
        }
    }
}

// Ways to place the tests
// 1) In the same file as lib.rs into a specific module
#[cfg(test)]            // Test's configuration
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    // error[E0433]         If you execute the test into the mod --> It doesn't see the previous public module
    // TODO: Check how to fix it
    // use OwnCrate::greetings;            // error[E0432]      Not fixed with this sentence. We get an error
    // #[test]
    // fn english_greeting_correct() {
    //     assert_eq!("hello", greetings::english::hello());
    // }
}

// 2) In the same file as lib.rs outside any module
#[test]
fn english_greeting_correct() {
    assert_eq!("hello", greetings::english::hello());
}

#[test]
#[should_panic]             // Test should fail
fn english_greeting_incorrect() {
    assert_eq!("hellods", greetings::english::hello());
}

#[test]
#[should_panic]             // Test should fail
#[ignore]                   // Test should be ignored === not runned
fn english_greeting_ignored() {
    assert_eq!("hellods", greetings::english::hello());
}
