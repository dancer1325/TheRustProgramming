#[warn(unused_variables)]

// Borrow an array without being able to modify it
fn use_slice_no_mutable(slice: &[i32]) {
    println!("No mutable. First elem is {}, len = {}", slice[0], slice.len());
}

// Borrow an array being able to modify it
fn use_slice(slice: &mut [i32]) {
    println!("Mutable. First elem is {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
    // will crash
    //let z = slice[10];
}

pub fn demo() {
    // a slice is part of an array
    // its size is not known at compile time
    let mut data = [1, 2, 3, 4, 5];
    println!("data before slice = {:?}", data);

    // start w/o mut, borrow as a slice
    use_slice_no_mutable(&data);            // No mutable
    use_slice(&mut data[1..4]);             // Part of the array
    use_slice(&mut data);                   // Entire array
    println!("data after slice use = {:?}", data);
}