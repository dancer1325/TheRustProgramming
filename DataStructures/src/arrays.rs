#[warn(unused_variables)]

use std::mem;
pub fn demo() {
    // Declare the array
    let mut aDeclared:[i32;5];      // [DataType; NumberOfElements]
    aDeclared = [2, 4, 5 ,6, 7];
    // println!(aDeclared);         // array can't be printed out directly

    // Initialize the array directly. Exact type is inferred
    let mut a/*:[i32;5]*/ = [1, 2, 3, 4, 5];
    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[0] = 321;         // Modify one of the elements
    println!("first value of a is {}", a[0]);

    // Comparing arrays
    // 1) By typical operators. Size must match
    if a != [1, 2, 3, 5, 6]
    {
        println!("arrays not equal!");
    }
    // If size doesn't match --> It doesn't compile
    // if a != [2, 3, 5, 6]
    // {
    //     println!("arrays not equal!");
    // }
    // 2) Use special operator
    assert_eq!(a, [321, 2, 3, 4, 5]);

    // Fill an array with 1s
    let b = [1u16; 10];         // Specifying the data type
    let c = [1; 10];            // Using the default type

    // Ways to get access to print all elements of the array
    // 1) For loop
    for i in 0..b.len() {           // First position of the array is 0
        println!("{}", b[i]);
    }
    // 2) :?
    println!("{:?}", b);

    // Size in memory of the array
    println!("b took up {} bytes", mem::size_of_val(&b));

    // multidimensional array
    let mtx: [[f32; 3]; 2] = [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0]];        // Declare an array of array
    println!("{:?}", mtx);

    // print all the diagonal values
    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}