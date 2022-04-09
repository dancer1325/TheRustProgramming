#[warn(unused_variables)]

//
enum Color {
    Red,                            // unit-like struct
    Green,
    Blue,
    RgbColor(u8, u8, u8),           // tuple struct
    CmykColor {                     // struct
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}
pub fn demo() {

    // Create an instance, choosing an option
    let c = Color::CmykColor {
        cyan: 0,
        magenta: 128,
        yellow: 0,
        black: 255,
    };
    //Color::RgbColor(0,0,0);
    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0)            // Possible in the matching to use binary operators
        | Color::CmykColor {
            cyan: _,                        // _  means that it can be anything
            magenta: _,                     // _  means that it can be anything
            yellow: _,                      // _  means that it can be anything
            black: 255,
        } => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({},{},{}", r, g, b),          // r, g, b  It means that any value is acceptable
        _ => (),                            // Default option
    }
}