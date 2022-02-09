// For allowing or not allowing
#![allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
// If you wouldn't add --> We would get warnings in the compiling processes
// #![allow(non_snake_case)] // Just allowed to put previous to the allowed

use std::mem;
// Way to import packages to use
// mem  Package about memory handling

fn main() {
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

}
