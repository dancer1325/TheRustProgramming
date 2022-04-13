trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("format - i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("format - String: {}", *self)
    }
}

// Passing a trait as argument by bound syntax
// Static dispatch
fn print_it<T: Printable>(z: T) {
    // monomorphisation     Determine in compilation time, which format function to call
    println!("print_it - {}", z.format());
}

// Passing a trait as argument by bound syntax
// Dynamic dispatch
fn print_it_two(z: &dyn Printable) {
    // monomorphisation     Determine in compilation time, which format function to call
    println!("print_it_two - {}", z.format());
}

pub fn demo() {
    let number = 123;
    let string = "hello".to_string();

    println!("{} with format {}", number, number.format());
    println!("{} with format {}", string, string.format());

    print_it(number);
    print_it(string);

    print_it_two(&number);
    // print_it_two(&string);          // error[E0382]:     TODO: Fix it why it's not possible to use or necessary to move
}