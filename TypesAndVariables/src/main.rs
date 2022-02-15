// For allowing or not allowing
#![allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
// If you wouldn't add --> We would get warnings in the compiling processes
// #![allow(non_snake_case)] // Just allowed to put previous to the allowed

mod stackandheap;  // Import module in the same directory. TODO: Why do we get a code time error?
use std::mem;
// Way to import packages to use
// mem  Package about memory handling

// Declare global variables
// 1) const
const MEANING_OF_LIFE:u8 = 42;
// 2) static
static static_immutable:u8 = 2;     // Immutable
static mut static_mutable:u8 = 2;  // Mutable

fn main() {
    // fundamental_data_types();
    // scope_and_shadowing();
    // operators();
    // unsafe {global_variables();} // If you use global mutable static variable --> Necessary to: 1) add unsafe function or 2) Catch it
    stackandheap::stack_and_heap(); // Invoke to another module
}

fn fundamental_data_types() {
    let x= 0;
    // Doing on purpose to test the allowed functionality

    let a: u8 = 123;
    // u8 comes from unsigned 8 bits --> [0, 255]
    // === non negative integer, since 2 bytes === 16 bits is a short integer
    // a refers to a place in memory to store this value

    // a = 453;
    // let declare a variable immutable
    // By default all the variables declared in Rust are immutable

    println!("a = {}", a);
    // Due to ! --> It's a macro
    // "{}" is to replace the value of the second argument

    let mut b : i8 = 0;
    // mut declare the variable as mutable
    // i8 comes from signed 8 bits --> [-128, 127]
    println!("b = {} before", b);
    b = 42;
    println!("b = {} after", b);


    // Type inference
    let c = 123456789;
    println!("c= {}, takes up {} bytes", c, mem::size_of_val(&c));
    //&c  Pointer to the variable c
    // 32 bits = 4 bytes

    // Declare integer variables native to the processor
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z); // Type is inferred, but it's usize
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z*8);
    // Why to *8? 1 byte := 2^8 bits

    // Characters
    let d: char = 'x';
    println!("{} is a char, size = {} bytes", d, mem::size_of_val(&d));

    // Floating point number
    let e:f32 = 2.5;
    println!("{} , size = {} bytes", e, mem::size_of_val(&e));

    let f:f64 = 2.5;
    println!("{} , size = {} bytes", f, mem::size_of_val(&f));

    // double floating point is the default floating point
    let g = 2.5;
    println!("{} , size = {} bytes", g, mem::size_of_val(&g));

    // Booleans
    let h: bool = false;
    println!("{} , size = {} bytes", h, mem::size_of_val(&h));

    let i: bool = true;
    println!("{} , size = {} bytes", i, mem::size_of_val(&i));
}

fn scope_and_shadowing() {
    let a = 123;
    let a = 1234; // It's possible to redeclare the variable

    println!("outside previous to the scoping, a = {}", a);
    // Create a scope just using {}
    {
        let b = 456;
        println!("inside, b = {}", b);

        println!("inside previous to declare it, a = {}", a); // Shadowing effect, because the variable exists in the scope since it has been declared in the parent scope
        let a = 777;
        println!("inside, a = {}", a);
    }

    println!("outside before the scoping, a = {}", a); // Value has been changed just into the scope
    // println!("inside, b = {}", b); // Since b isn't declared in this scope --> it throws an error
}

fn operators() {

    // Priority order is the mathematical one
    let mut a = 2+3*4;
    println!("a: {}", a);

    // Not supported -- nor ++
    // a++
    // a--

    // But it's supported the combination with =
    a-=2;
    println!("Substract 2 to a:{}", a);

    a+=1;
    println!("Add 1 to a:{}", a);

    a*=2;
    println!("Multiplying by 2, to a:{}", a);

    a/=3;
    println!("Dividing by 3, to a:{}", a);

    a%=3;
    println!("Remainder, dividing by 3, to a:{}", a);

    // ^ not supported --> Necessary to use a function
    // If either base are power are integer
    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    // If the base isn't integer
    let b = 2.5;
    let b_cubed = f64::powi(b, 3); // i in powi, because the power is an integer
    let b_to_pi = f64::powf(b, std::f64::consts::PI); // f in powf, because the power is floating point    PI is in chapitals because it's a const
    println!("b {} cubed is {}", b, b_cubed);
    println!("b {} ^ pi is {}", b, b_to_pi);

    // bitwise operators  --> Operations will be done with the binary expression
    // Just valid for integers.

    // |
    let c = 1 | 2;
    // 1 --> 01, 2 --> 10,
    // 01 | 10 = 11 --> 3
    println!{"1|2 is {}", c};

    // &
    let d = 1 & 2;
    // 1 --> 01, 2 --> 10,
    // 01 & 10 = 00 --> 0
    println!{"1&2 is {}", d};

    // ^
    let e = 1 ^ 2;
    // 1 --> 01, 2 --> 10,
    // 01 ^ 10 = 11 --> 3
    println!{"1^2 is {}", e};

    // !
    // TODO: How is the NOR operator ?
    // let f = 1!2;  // It's not correct
    // let f = !(1|2);
    // 1 --> 01, 2 --> 10,
    // 01 ! 10 = 00 --> 0
    // println!{"1!2 is {}", f};

    // shift operators. They are part of bitwise operators
    // Its first argument is a decimal number
    let g = 2 << 2;
    println!{"2 << 2 is {}", g};

    let h = 2 >> 2;
    println!{"2 >> 2 is {}", h};

    // logical operators.
    // < , > , == , <= , >=
    let pi_less_4 = std::f64::consts::PI < 4.0;
    println!{"pi_less_4 is {}", pi_less_4}


}

// If you use mutable static --> Necessary to: 1) add unsafe function or 2) Catch it
unsafe fn global_variables() {
    println!{"MEANING_OF_LIFE is {}", MEANING_OF_LIFE};
    println!{"z is {}", static_immutable};
    println!{"y is {}", static_mutable};
}
