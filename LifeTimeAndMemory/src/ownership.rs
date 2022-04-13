
pub fn demo() {
    // Variables moved

    // 1) Vector
    let v = vec![1, 2, 3];              // Pointer to the data

    // Assignments
    // A] It doesn't take memory of v and copy to v2
    // B] A pointer is copied effectively, but just v2 binds / points to the data, and v isn't longer usable
    // let v2 = v;

    // You can check that v isn't longer usable, because it throws an error
    // println!("{:?}", v);             // error[E0382]

    // 2) Closures
    let foo = |v:Vec<i32>| ();
    // foo(v);

    // println!("{:?}", v);                // error[E0382]

    // It doesn't happen with primitive data
    let u = 1;          // i32      It's not a pointer to the data. It's a chunk of memory storing the information
    let u1 = u;         // It's doing a copy instead of moving
    println!("u = {}", u);

    // If you wrap the primitive data --> You have a pointer
    let w = Box::new(1);
    let w2 = w;
    // println!("w = {}", w);                 // error[E0382]

    // Returning the argument passed
    let print_vector = |x:Vec<i32>| -> Vec<i32> {
        println!{"{:?}", x};
        x
    };
    let vv = print_vector(v);
    println!("{}", vv[0]);
    // println!("{}", v[0]);               // error[E0382]
}