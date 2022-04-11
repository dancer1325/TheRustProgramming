use std::collections::HashMap;
pub fn demo() {
    let mut shapes = HashMap::new();
    // types that implement the Copy trait are ok
    // types that don't are moved
    let triangle = String::from("triangle");
    shapes.insert(triangle, 3);
    shapes.insert("square".into(), 4);          // "square".into()   Convert and creates a string
    //let t = triangle;

    // Get the value based on the key
    // into It's a trait to convert to another type
    println!("a square has {} sides", shapes["square".into()]);

    // iterate the entire HashMap
    // Indexes are the key and value
    for (key, value) in &shapes {
        println!("{} : {}", key, value);
    }

    // can overwrite values
    shapes.insert("square".into(), 5);
    println!("{:?}", shapes);


    let e = shapes.entry("square".into());
    // upsert (only insert if it has no value)
    shapes.entry("circle".into()).or_insert(1);         // Add the value in case the key doesn't exist
    println!("shapes after or_insert operation with 1 {:?}", shapes);
    {
        let actual = shapes.entry("circle".into()).or_insert(2);
        // println!("shapes after or_insert operation with 2 {:?}", shapes);           // Not possible to use a variable as mutable and immutable at the same time
        *actual = 0;            // Modify the value
        println!("shapes after updating the value with 0 {:?}", shapes);
    }
    println!("shapes after all operations {:?}", shapes);
}