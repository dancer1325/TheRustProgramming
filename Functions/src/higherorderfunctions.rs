fn is_even(x: u32) -> bool {
    x % 2 == 0
}

// function that return functions
fn greater_than(limit: u32) -> impl Fn(u32) -> bool {               // Fn           It's a trait, implement automatically by closures
    // |y| y > limit                                                   // error[E0373]     Required that closure take ownership of the variable
    move |y| y > limit
}

pub fn demo() {
    // functions that take functions
    // sum of all even squares <= 500
    let limit = 500;
    let mut sum = 0;
    // Ways to check if the limit is overcome
    // 1) Normal closure
    //let above_limit_2 = |y| y > limit;
    // 2) Higher function
    let above_limit = greater_than(limit);
    for i in 0.. {
        let isq = i * i;
        //if isq > limit {                       // Without using functions
        if above_limit(isq) {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }
    println!("loop sum = {}", sum);

    // Same as previous, but using some Rust operators
    let sum2 = (0..)                            // Infinite sequence
        .map(|x| x * x)
        .take_while(|&x| x < limit)             // Requires the reference === memory address
        .filter(|x| is_even(*x))                // Require the value
        .fold(0, |sum, x| sum + x);             // Flatten all the elements. First argument is the initial value. Second argument is a closure with arguments the accumulator and the next element
    println!("hof sum = {}", sum2);
}
