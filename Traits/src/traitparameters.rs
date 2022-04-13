#![allow(dead_code)]
#![allow(unused_imports)]
use std::fmt::Debug;                    // It's another trait. Format the output in a programmer-facing, debugging context

#[derive(Debug)]                        // Implement a in-built Rust trait to a struct
struct Circle {
    radius: f64,
}
#[derive(Debug)]                        // Implement a in-built Rust trait to a struct
struct Square {
    side: f64,
}

// Trait which it's implemented by Square and Circle
trait Shape {
    fn area(&self) -> f64;                        //self      Points to the current class
}
impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

// Ways to specify that the function's argument is a trait parameter

// 1) Indicating in the argument that it's an impl of the trait
fn print_info_indicating_argument(shape: impl Shape + Debug) {          // All the traits must be indicated
    println!("print_info_indicating_argument - {:?}", shape);
    println!("print_info_indicating_argument - The area is {}", shape.area());
}

// 2) Trait bound syntax
// Advantages: A] Several arguments with same traits, not necessary to define the traits for each one
//fn print_info<T: Shape + Debug>(shape1: T, shape2: T)             // If there are several arguments with the same traits
fn print_info_bound_syntax<T: Shape + Debug>(shape: T) {            // <>       Indicate the traits of each generic
    println!("print_info_bound_syntax - {:?}", shape);
    println!("print_info_bound_syntax - The area is {}", shape.area());
}


// 3) where
fn print_info_where<T>(shape: T)
    where T: Shape + Debug                          // Specify all the traits here
{
    println!("print_info_where - {:?}", shape);
    println!("print_info_where - The area is {}", shape.area());
}

pub fn demo() {
    let circle = Circle { radius: 2.0 };
    let square = Square { side: 2.0 };

    // error[E0382]: TODO: Fix it, because if not, it's just possible to invoke 1 couple of functions per time
    // print_info_indicating_argument(circle);
    // print_info_indicating_argument(square);

    // error[E0382]: TODO: Fix it, because if not, it's just possible to invoke 1 couple of functions per time
    // print_info_bound_syntax(circle);
    // print_info_bound_syntax(square);

    print_info_where(circle);
    print_info_where(square);
}