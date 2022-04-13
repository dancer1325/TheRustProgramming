pub fn demo() {
    // let print_vector = |x:&Vec<i32>| -> Vec<i32> {       // Unnecessary to return
    let print_vector = |x: &Vec<i32>| {
        println! {"{:?}", x};
    };

    let v = vec![3, 2, 1];
    print_vector(&v);                    // &       Not give all the control of v, just giving the control during the execution of the function
    println!("v[0] = {}", v[0]);         // Possible to reuse v, because it hasn't been moved

    let mut a = 40;
    // let mut b = &mut a;         // Borrow the control to a
    // println!("b = {}", b);
    // println!("a = {}", a);              // error[E0502]

    // Commenting the next lines, previous one works
    // *b += 2;
    // println!("b = {}", b);
    // println!("a = {}", a);

    // Solution
    // Wrapping into a block all about b, to release a after closing the block
    println!("a = {}", a);
    {
        let mut b = &mut a;         // Borrow the control to a. b and a must have got the same mutability
        *b += 2;
        println!("b = {}", b);
    }
    println!("a = {}", a);                  // It's value has been changed, because b was changed

    let mut z = vec![3, 2, 1];
    for i in &z {
        println!("i = {}", i);
        // z.push(5);                          // error[E0502]     Rust prevents us errors, like infinite loop in this case
    }
}