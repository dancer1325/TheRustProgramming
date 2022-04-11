use std::collections::HashSet;
pub fn demo() {
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}", greeks);

    // If you try to add the same element several times, just 1 will be added
    greeks.insert("delta");
    println!("{:?}", greeks);

    // Insert returns a boolean
    let added_epsilon = greeks.insert("epsilon");
    let added_gamma = greeks.insert("gamma");
    if added_epsilon { println!("added_epsilon: {}", added_epsilon) };
    if added_gamma { println!("added_gamma: {} ", added_gamma) } else { println!("added_gamma: {} ", added_gamma)  };

    // contains    Check if the key exists
    if greeks.contains("gamma") {println!("gamma is contained ")};

    // removed     Remove an entry in the HashSet, returning a boolean in case it was or not removed
    let removed = greeks.remove("gamma");
    if removed { println!("gamma was removed ") } else { println!("gamma didn't exist ") };

    // Type is inferred
    let _1_5: HashSet<_> = (1..5).collect();            // No inclusive the highest one
    let _6_10: HashSet<_> = (6..10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();         // Inclusive the highest one
    let _2_5: HashSet<_> = (2..=5).collect();

    println!("_1_5 {:?}", _1_5);                        // The order is random
    println!("_6_10 {:?}", _6_10);
    println!("_1_10 {:?}", _1_10);
    println!("_2_5 {:?}", _2_5);

    // is_subset()
    // let is1To5SubsetOf1To10 = _1_5.is_subset(_1_10);             // Necessary to use the pointer
    let is1To5SubsetOf1To10 = _1_5.is_subset(&_1_10);
    println!("_1_5 is subset of _1_10: {} ", is1To5SubsetOf1To10);

    //is_disjoint()
    // Check that they don't have common elements
    let is1To5DisjointOf1To10 = _1_5.is_disjoint(&_1_10);
    println!("_1_5 is subset of _1_10: {} ", is1To5DisjointOf1To10);

    // union
    let is1To5UnionOf1To10 = _1_5.union(&_1_10);
    println!("_1_5 is union of _1_10: {:?} ", is1To5UnionOf1To10);

    // intersection
    let is1To5IntersectionOf1To10 = _1_5.intersection(&_1_10);
    println!("_1_5 is intersection of _1_10: {:?} ", is1To5IntersectionOf1To10);

    // difference
    // symmetric difference = union - intersection
}