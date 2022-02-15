#![allow(dead_code)]

use std::mem;

// Define an object === Golang
struct Point {
    x: f64,
    y: f64
}

// Function returning a Point
fn origin() -> Point {
    Point{x:0.0, y:0.0}
}

pub fn stack_and_heap() {

    // Stack allocated
    let p1 = origin();
    println!("p1 takes up {} bytes", mem::size_of_val(&p1)); // The size is the addition of x (64 bits = 6 bytes) and y (64 bits = 6 bytes)

    // Heap allocated
    let p2 = Box::new(origin());
    println!("p2 takes up {} bytes", mem::size_of_val(&p2)); // The size is the size of the address in the memory, which is platform specific. Example: 64-bits === 8 bytes

    // From pointer --> Get the value
    let p3 = *p2;
    println!("p2 has got values: x {} and y {}", p3.x, p3.y);
}