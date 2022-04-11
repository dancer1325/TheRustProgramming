pub fn demo() {

    // Ways to initialize a Vector
    // 1) Constructor
    // let a = Vec::new();              // It wouldn't compile if we don't do it mut, because we add values
    let mut vector = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);
    println!("vector = {:?}", vector);

    // 2) vec!
    let mut a = vec![1, 2, 3]; // [1;10]
    println!("a = {:?}", a);

    // Get vector's element
    // 1) [Index]
    // let idxWrong:i32 = 0;
    // println!("a[0] = {}", a[idxWrong]);
    // iSize variables can't be used as vector's index, because
    // A] Memory address can't be negative, B] If you use a 64 machine, no sense to use 32 variable

    let idx/*:i32*/ = 0; // will not work with :i32
    // you need usize
    println!("a[0] = {}", a[idx]);

    // unsafe access. If the element in that position doesn't exist --> Error is thrown
    //println!("a[5] = {}", a[5]);

    // 2) get
    // If the index doesn't exist --> Error isn't thrown. Return Options
    match a.get(5) {
        Some(x) => println!("a[5] = {}", x),            // Way to get the element is with []
        None => println!("error, no such element"),
    }


    // iterate
    for x in &a {
        println!("{}", x);
    }

    // adding/removing
    // Stack memory
    a.push(44);
    println!("{:?}", a);
    let last_elem = a.pop(); // can easily yield nothing. Popping the element from the end, returning an Option --> It doesn't crash
    println!("last elem is {:?}, a = {:?}", last_elem, a);
    // explain why this doesn't work
    //let Some(last_value) = a.pop();
    // print the elements in reverse order

    // Necessary to use a condition, because pop could return None
    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}