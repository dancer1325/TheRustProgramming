pub fn demo() {
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        if x == 64 {
            continue; // Next line into the while loop isn't executed for this element of iteration, and it jumps to the next element into the iteration list
        }
        println!("x = {}", x);
    }

    let mut y = 1;
    loop // while true === infinite loop, at least you indicate it explicitly with break statement
    {
        y *= 2;
        println!("y = {}", y);
        // stop at 2^10, since we are always multiplying in this case by 2
        if y == 1 << 10 {
            break;
        }
    }
}