struct Circle { radius: f64 }
struct Square { side: f64 }
trait Shape {
    fn area(&self) -> f64;
}
impl Shape for Square {
    fn area(&self) -> f64
    {
        self.side * self.side
    }
}
impl Shape for Circle {
    fn area(&self) -> f64
    {
        self.radius * self.radius * std::f64::consts::PI
    }
}
pub fn demo()
{
    // let shapes:[&Shape; 4] = [                  // error[E0782]:
    let shapes:[&dyn Shape; 4] = [
        &Circle{radius: 1.0},
        &Square{side: 3.0},
        &Circle{radius: 2.0},
        &Square{side: 4.0}
    ];
    for (i, shape) in shapes.iter().enumerate()
    {
        // Dynamic dispatch is the only way to distinguish which one to use
        println!("Shape #{} has area {}", i, shape.area());
    }
}
