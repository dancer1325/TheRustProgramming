#[warn(unused_variables)]

// we don't know whether this contains an int or a float
// allocate in this case 32 bits in memory
union IntOrFloat {
    i: i32,
    f: f32,
}
fn process_value(iof: IntOrFloat) {
    unsafe {                    // Required because Rust doesn't know the chosen option in the union
        match iof {
            IntOrFloat { i: 42 } => {
                println!("meaning of life value");
            }
            //           ↓↓↓ we get to treat it as f32
            IntOrFloat { f } => {
                // we don't know if it's int or float
                println!("got some value which could be a float {}", f);
            }
        }
    }
}
pub fn demo() {
    // Create an instance, choosing integer
    let mut iof = IntOrFloat { i: 123 };
    // Rust doesn't know the chosen value (integer or float)
    // Cannot access member without an unsafe block
    let ivalue = unsafe { iof.i };
    let fvalue = unsafe { iof.f };
    println!("iof with iof.i {} and iof.f {}", ivalue, fvalue);
    // Since it doesn't match with the first condition of the process_value --> iof is casted to floating pont
    process_value(iof);

    iof.i = 234;
    // Rust doesn't know the chosen value (integer or float)
    // Cannot access member without an unsafe block
    let ivalue = unsafe { iof.i };
    let fvalue = unsafe { iof.f };
    println!("iof with iof.i {} and iof.f {}", ivalue, fvalue);
    // Since it doesn't match with the first condition of the process_value --> iof is casted to floating pont
    process_value(iof);

    // If you define a floating point is completely different to integer
    let iof2 = IntOrFloat { f: 42.0 };
    // Rust doesn't know the chosen value (integer or float)
    // Cannot access member without an unsafe block
    let ivalue = unsafe { iof2.i };
    let fvalue = unsafe { iof2.f };
    println!("iof with iof.i {} and iof.f {}", ivalue, fvalue);
    process_value(iof2);

    let iof3 = IntOrFloat { i: 42 };
    // Rust doesn't know the chosen value (integer or float)
    // Cannot access member without an unsafe block
    let ivalue = unsafe { iof3.i };
    let fvalue = unsafe { iof3.f };
    println!("iof with iof.i {} and iof.f {}", ivalue, fvalue);
    process_value(iof3);

    // this will interpret an int as a float
    let iof4 = IntOrFloat { i: 123456 };
    // Rust doesn't know the chosen value (integer or float)
    // Cannot access member without an unsafe block
    let ivalue = unsafe { iof4.i };
    let fvalue = unsafe { iof4.f };
    println!("iof with iof.i {} and iof.f {}", ivalue, fvalue);
    // Since it doesn't match with the first condition of the process_value --> iof is casted to floating pont
    process_value(iof4);
}