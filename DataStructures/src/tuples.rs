#[warn(unused_variables)]

// Return a tuple
fn sum_and_product(x: i32, y: i32) -> (i32, i32, bool) {
    (x + y, x * y, true)
}

pub fn demo() {
    let x = 3;
    let y = 4;

    let sp = sum_and_product(3, 4);             // Not required in functions to pass all the arguments
    println!("sp = {:?}", sp);              // :?  can be used to loop through all elements in the tuple
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1); // Get access to tuples' elements
    // println!("sp.4 = {} ", sp.4);                                // Getting an error if you try to look for an element which doesn't exist


    // destructuring
    // let (a, b) = sp;                             // Different to dynamic type languages (such as JS), it's necessary to indicate all elements
    let (a, b, c) = sp;
    println!("a = {}, b = {}", a, b);

    // tuple of tuples
    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("last element is {}", (combined.1).1);

    // Destructuring the tuple of tuple
    let ((c, d, g), (e, f, h)) = combined;

    // tuple of different elements
    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);

    // tuple of a single element
    // let notuple = (42);     // It's not a tuple. It's a simple integer
    // println!(notuple);
    let meanings = (42,); // start w/o comma
    println!("{:?}", meanings);
}