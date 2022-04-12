pub fn demo() {
    // Types of String
    // 1) string slice
    // a view into a String
    let s: &'static str = "hi there!"; // &'static str -->
    // statically allocated (part of the program)
    // s = "bar"; // cannot reassign immutable

    // Get the characters
    // A) Not possible to get them directly
    //let a = s[0];
    // B) .chars()   Get access to the sequence of characters
    for c in s.chars()
    {
        println!("{}", c);
    }
    // C) .nth()            Get the nth character of the string
    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter is {}", first_char)
    }

    // Manipulation can be done just by characters, not by complete string
    // A) .rev()            Reverse the characters
    for c in s.chars().rev() {
        // reversed! also as_bytes()
        println!("{}", c);
    }
    // B) .replace()
    println!("{} and once replaced {}", s, s.replace("hi", "goodbye"));

    // 2) String
    // heap allocated construct
    // Vec<u8>, guaranteed to be valid UTF-8
    let mut letters = String::new();                // mut      Required to use the String property that it's mutable
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");                      // _str     Append not characters (such as ,) into String
        a = a + 1;                                  // alphabetic letters by position
    }
    println!("{}", letters);
    // Other manipulations
    letters.remove(0);
    letters.push_str("!!!");

    // Conversion between string's types
    // A) String --> &str
    let u: &str = &letters; // deref conversion
    // there are situations when the coercion does NOT happen
    // B) str --> String
    // B.1] .from()
    let mut abcFrom = String::from("hello world");
    // B.2] .to_string()
    let mut abcToString = "hello world".to_string();

    // Other manipulations
    // .replace()
    println!("{} and once replaced {}", abcFrom, abcFrom.replace("hello", "goodbye"));

    // concatenation
    // String + str
    let z = letters + "abc";
    // String + &String
    // let zSecond = letters + &letters;         // Required to convert to str the second one

    // format!          It's a macro
    let name = "Alfredo";
    let surName = "Toledano";
    let greeting = format!("Hello, I'm {} {}, nice to meet you", name, surName);       // Several arguments can be passed by position
    println!("{}", greeting);

    let greetingTwo = format!("Hello, I'm {0} {1}, nice to meet you. My name is {0}, and my surname is {1}", name, surName);       // Positions can be indicated, to avoid repeating arguments
    println!("{}", greetingTwo);

    let bond = format!("My name is {name}, {name} {surname}", name = "James", surname = "Bond");       // Indicating the reference
    println!("{}", bond);

    let mixed = format!("{1} {} {0} {} {data}", "alpha", "beta", data = "delta");   // Possible to mix the previous ways of formatting
    // {}       Although you have specified previously a value by the position, it's got it from the scratch
    println!("{}", mixed);

    // let mixedWithWrongNumberOfElements = format!("{1} {} {0} {} {data}", "alpha", "beta", "gamma", data = "delta");     // If you specify more than necessary arguments --> Compiler throws an error
    // println!("{}", mixedWithWrongNumberOfElements);
}
