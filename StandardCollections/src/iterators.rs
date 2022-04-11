pub fn demo()
{
    let mut vec = vec![3, 2, 1];

    // 1) Going through all the elements directly
    // ordinary iteration causes a move
    // for x in vec
    // {
    //     println!("{}", x);
    // }
    // vec.iter();                  // It has been the index of iteration to the end --> It can't be used

    // 2) By reference
    for x in &vec
    {
        println!("{}", *x);         // Through all the elements by reference
    }

    // 3) iter()
    // A bunch of immutable references
    for x in vec.iter()
    {
        println!("we got {}", x);               // Not necessary to use *
        // cannot modify things! since it's immutable
        // x += 1;
    }

    // iter adapter methods
    // .rev()           Revert the order of the elements
    for x in vec.iter().rev()
    {
        println!("in reverse: {}", x);
    }

    // Mutable iterations. Necessary
    // 1) Mutable vector
    // 2) Mutable iteration method
    // iter_mut() - mutable references, requires
    //              the vector to be declared mut
    for x in vec.iter_mut()
    {
        *x += 2;
    }
    println!("{:?}", vec);

        // into_iter() - move operation that transforms the collection --> by-value iterator
    //               not the same as ordinary iteration!
    //               useful when you need values but not the collection itself
    // extend() - automatically calls into_iter() to move elements from one collection to another
    let mut vec2 = vec![1, 2, 3];
    vec2.extend(vec);
    println!("{:?}", vec2);
}