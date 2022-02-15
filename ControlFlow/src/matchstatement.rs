pub fn demo() {

    // Matching strings
    let country_code = 50;
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown", // inclusive range ... is deprecated

        _ => "invalid",        // Default case, if it doesn't no match with some previous one.
        // If we comment it and the value to match isn't covered by any previous one --> It will throw a compilation error
    };
    println!("the country with code {} is {}", country_code, country);

    // Matching booleans
    let x = false;
    let s = match x {
        true => "yes",
        false => "no",
    };
    println!("The value selected {} is {}", x, s);
}