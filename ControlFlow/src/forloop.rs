pub fn demo() {

    // for  in
    for x in 1..11 // Last element isn't taken --> Till 10 just
    // for x in 1..=10 // Equivalent to the previous iteration
    // 1 to 10; 11..1 won't work
    {
        // skip 3
        if x == 3 {
            continue;
        }
        // stop at 7
        if x == 8 {
            break;
        }
        println!("x = {}", x);
    }

    // x are the own elements in the iteration. They aren't indexOfIteration
    for x in 30..41 {
        println!("x = {}", x);
    }

    // Descending order ?
    // It doesn't throw a compilation error, but it doesn't get as descending order, and it understands that the length is 0
    for x in 40..30 {
        println!("Descending order. x = {}", x);
    }

    // enumerate
    // Add 2 entries in the for section: (indexOfIteration, element)
    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }

    // enumerate
    // TODO: If you add 1 entry in the for section?
    // for pos in (30..32).enumerate() {
    //     // println!("{}:", pos.toString); // TODO: How to get access to its value?
    //     // println!("{}:", pos);
    //     println!("{}", std::any::type_name::<pos>()); // TODO: How to get the type of an object?
    // }
}