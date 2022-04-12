fn say_hello() {
    println!("hello");
}
pub fn demo() {
    // Invoke a function
    say_hello();

    // Store a function in a variable, and afterwards to invoke it
    let sh = say_hello;
    sh();

    // Closure
    // Define a function directly inline, without using keyword fn
    let plus_one = |x: i32| -> i32 { x + 1 };            // Specifying the type of the argument
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    // Without scoping
    let mut two = 2;
    let plus_two = |x| {                               // Type of the argument can be inferred
        let mut z = x;
        // z += 2;                                              // Indicating directly the value to add
        z += two;
        z                                                       // No ;  --> Value to return in the function
    };
    println!("{} + 2 = {}", 3, plus_two(3));
    let borrow_two = &mut two;                         // We could have problems to use two in 2 scenarios  A) By the closure,  B) By other

    // With scoping
    let mut two = 2;
    {                                                           // Scoping the closure, we avoid having problems, if we use a variable into a closure and in other scenario
        let plus_two = |x| {
            let mut z = x;
            z += two;
            z
        };
        println!("{} + 2 = {}", 3, plus_two(3));
    }
    let borrow_two = &mut two;

    // Ways to pass the arguments
    // 1)  T:               by value  === Copy of the variable
    let plus_three_by_value = |mut x: i32| x += 3;
    let mut f = 12;
    let returnedByValue = plus_three_by_value(f);
    // println!("f = {} and the returned by value {}", f, returnedByValue);             //TODO: Error displaying the format
    println!("f = {} by value", f);                 // IMPORTANT!!!: Here you can see that the variable's value hasn't been changed

    // 2) T&               by reference     === by memory address

    // 3) &mut &           by mutable reference
    let plus_three_by_reference = |x:&mut i32| *x += 3;               // mut      Because I want to change the argument
    // let f = 12;                                                 // If you declare a variable as immutable --> Afterwards it can't be passed as mutable
    let mut f = 12;
    // plus_three(& f);                                                 // mut      Required to indicate it, because if not, it throws an error
    // returnedByReference = plus_three_by_reference(&mut f);                              // TODO: error[E0425]
    plus_three_by_reference(&mut f);
    // println!("f = {} and the returned by reference {}", f, returnedByReference);         // TODO: Required to fix previous error
    println!("f = {} by reference", f);
}