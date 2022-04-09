#[warn(unused_variables)]
pub fn demo() {
    let x = 3.0;
    let y = 0.0;
    let result: Option<f64> = if y != 0.0 { Some(x / y) } else { None };
    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("cannot divide {} by {}", x, y),
    }

    // let
    // allows checking if what's on the right can be assigned what's on the left
    if let Some(z) = result {
        println!("Some(z) with z {}", z);
    }
    // Failure if you don't use let
    // else if Some(z) = result {
    //     println!("Some(z) with z {}", z);
    // }
    else if let None = result {
        println!("None");
    }
    // while let  It can be also used

    // Type inferred because it's an option although you don't specify it
    let resultWithoutOption =
        if y != 0.0 { Some(x / y) } else { None };

    match resultWithoutOption {
        Some(z) => println!("resultWithoutOption - {}/{}={}", x, y, z),
        None => println!("resultWithoutOption - cannot divide {} by {}", x, y),
    }
}