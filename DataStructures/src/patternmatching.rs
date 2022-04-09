#[warn(unused_variables)]

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // tuple
    CmykColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    }, // struct
}

// fn how_many(x: i32) -> static str {                  // Not allowed using 2 reserved keywords
// fn how_many(x: i32) -> 'static str {                 // Required to have a body. Not possible to use special characters directly
fn how_many(x: i32) -> &'static str {                   // & allows using special characters (such as ')
    // Once it enters in one option ⟶ It goes out of the match pattern
    match x {
        0 => "no",
        1 | 2 => "one or two",
        // 9..12 => "lots of",                             // Exclusive range not allowed in a match pattern
        9..=12 => "lots of",                             // Inclusive range
        12 => "a dozen",
        //z @ 20...30 => "between 20 and 30",
        _ if (x % 2 == 0) => "some even number of",     // _ If you aren't interested in the number. Conditions can be added
        _ => "a few",                                   // Default option
    }
}

pub fn demo() {
    for x in 0..23 {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    let point = (3, 0);
    // Pattern matching of tuples of elements can be used
    match point {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y = {}", y),            // y  Because any element can be used, but it's used afterwards
        // also try ref and ref mut
        (ref x, 0) => println!("y axis, x = {}", x),               // ref  Because you pass by reference the variables
        // (ref mut x, 0) => println!("y axis, x = {}", x),       // ref mut  Because you pass by reference and mutable the variables, but it's necessary that the variable is mut
        (_, y) => println!("(?,{})", y),
    }

    let c: Color = Color::CmykColor {
        cyan: 0,
        magenta: 128,
        yellow: 0,
        black: 255,
    };
    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        // If you are interested in just 1 property
        // Color::RgbColor(0, 0, 0) | Color::CmykColor { cyan: _, magenta: _, yellow: _, black: 255, .. } => println!("black"),
        Color::RgbColor(0, 0, 0) | Color::CmykColor { black: 255, .. } => println!("black"),            // .. If you aren't interested in the values of other properties
        Color::RgbColor(r, g, b) => println!("rgb({},{},{})", r, g, b),
        _ => (),
    }
}