use std::ops::{Add, AddAssign, Neg};
use std::cmp::{PartialEq};

#[derive(Debug)]                            // Implement a in-built Rust trait to a struct
// #[derive(Debug, PartialEq, Eq)]          // If you work with the default trait's implementation -->  Derive automatically, instead of manually
struct Complex<T> {
    re: T,
    im: T,
}

// impl Complex<T> {                       // error[E0412]:        If you don't specify the generic type
impl<T> Complex<T> {
    fn new(re: T, im: T) -> Complex<T> {
        // Complex<T> {re, im}             // It throws an error, because it doesn't understand <> as generic
        Complex::<T> { re, im }
    }
}

// Implement the trait to calculate +
// Important!!! Just possible to implement multiple times the same trait although, each one returns different types
// 1) i32
// impl Add for Complex<i32> {
//     type Output = Complex<i32>;                             // Specify the result type for add function
//
//     // self     Reference to the left member of the operation.
//     // Self    Reference to the left member of the operation. Also reference to the struct which implements this trait
//     fn add(self, other: Self) -> Self::Output {
//         Complex {
//             re: self.re + other.re,
//             im: self.im + other.im,
//         }
//     }
// }
// 2) Generic
impl<T> Add for Complex<T>
    where T: Add<Output=T>                                // Required to
{
    type Output = Complex<T>;
    // Specify the result type for add function
    // self     Reference to the left member of the operation.
    // Self    Reference to the left member of the operation. Also reference to the struct which implements this trait
    fn add(self, other: Self) -> Self::Output {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

// Implement the trait to calculate +=
impl<T> AddAssign for Complex<T>
    where T: AddAssign<T>                                // Required to
{
    // Specify the result type for add function
    // self     Reference to the left member of the operation.
    // Self    Reference to the left member of the operation. Also reference to the struct which implements this trait
    fn add_assign(&mut self, other: Self) {
        self.re += other.re;
        self.im += other.im;
    }
}

// Implement the trait to calculate -
impl<T> Neg for Complex<T>
    where T: Neg<Output=T>                                // Required to
{
    type Output = Complex<T>;
    // Specify the result type for add function
    // self     Reference to the left member of the operation.
    // Self     Reference to the struct which implements this trait
    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

//  Implement the trait to calculate == partially
impl<T> PartialEq for Complex<T>
    where T: PartialEq                                // Required to
{
    // self     Reference to the left member of the operation.
    // Self     Reference to the left member of the operation. Also, reference to the struct which implements this trait
    fn eq(&self, other: &Self) -> bool{
        self.re == other.re && self.im == other.im
    }
}

//  Implement the trait to calculate fully ==
impl<T: Eq> Eq for Complex<T>
    where T: Eq                                // Required to
{
    // No implementation is required
}

pub fn demo() {
    let mut a = Complex::new(1, 2);
    let mut b = Complex::new(3, 4);

    println!("a {:?}", a);
    println!("b {:?}", b);

    // If you don't define add trait as implementation for the struct --> error[E0369]:
    // This add works either 1) or 2) add implementation
    println!("a + b   {:?}", a + b);

    // let mut c = Complex::new(1.0, 2);            // error[E0308]:   Method with just 1 generic with the same type
    let mut c = Complex::new(1.0, 2.0);
    let mut d = Complex::new(3.0, 4.0);

    println!("c {:?}", c);
    println!("d {:?}", d);

    // If you don't define add trait as implementation for the struct --> error[E0369]:
    // This add just works for 2) add implementation
    println!("c + d {:?}", c + d);

    let mut e = Complex::new(1.0, 2.0);
    let mut f = Complex::new(3.0, 4.0);
    // += operator
    e += f;
    println!("e {:?}", e);

    // - operator
    println!("-e {:?}", -e);

    let mut g = Complex::new(3.0, 4.0);
    let mut h = Complex::new(3.0, 4.0);
    let mut i = Complex::new(3.0, 6.0);
    // == operator
    println!("g == h :  {:?}", g == h);
    println!("g == i :  {:?}", g == i);
}