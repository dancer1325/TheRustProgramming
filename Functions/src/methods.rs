#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}
struct Line {
    start: Point,
    end: Point,
}
impl Line {                                     // Structs' function
    // fn len(&self) {                          // If you don't specify the value to return --> It throws an error
    fn len(&self) -> f64 {                      // &self        Memory address to the class in which you are using it
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        return (dx * dx + dy * dy).sqrt();              // Pitagoras' theorem
    }
    // fn hello() {                             // If you don't specify self, although you don't use it --> It throws an error
    fn hello(&self) {
        println!("Hello, from Line");
    }
}
pub fn demo() {
    let p = Point { x: 3.0, y: 4.0 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let myline = Line { start: p, end: p2 };

    println!("length = {}", myline.len());
    myline.hello();
}