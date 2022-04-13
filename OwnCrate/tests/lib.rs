// 3) In a specific folder in a lib.rs
#[cfg(test)]            // Test's configuration
mod tests {

    // extern crate OwnCrate;           // Not required, since it's all in the same crate
    use OwnCrate::greetings;            // Required to indicate the mod to use

    #[test]
    #[should_panic]             // Test should fail
    fn english_greeting_incorrect() {
        assert_eq!("hellods", greetings::english::hello());
    }

}