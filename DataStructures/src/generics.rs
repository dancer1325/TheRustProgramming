#[warn(unused_variables)]
struct Point<T> {
    x: T,
    // Without specifying the type
    y: T,           // Without specifying the type
}

// Struct with several Generics
struct PointWithSeveralGenerics<T, V> {
    x: T,
    y: V,
}

// Nested generic structs can be used
struct Line<T> {
    start: Point<T>,
    end: Point<T>,
}

pub fn demo() {
    let a = Point { x: 0, y: 0 };

    // If you initialize another variable with different data types is fine
    let b = Point { x: 1.2, y: 3.4 };

    // If you define 1 Generic type in the struct --> Both have got the same one
    // let c = Point { x: 1, y: 3.2 };              // It throws an error

    // Defining a struct with several generics
    let d = PointWithSeveralGenerics { x: 1, y: 3.2 };

    // won't work initially
    let e = Point { x: 10, y: 20 };
    let myline = Line { start: a, end: e };
}



