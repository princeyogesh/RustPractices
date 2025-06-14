/*
    Match operator is similar to Switch in C++ or Select in Go
    Match also has fallthrough mechanism like C++ switch

 */
#[derive(Debug)]
enum Shape {
    Circle(f64), //radius
    Rectangle(f64, f64), //length and breadth
    Triangle(f64, f64, f64) // sides of triangle
}
fn main() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my shape is {:?}", my_shape);

    match my_shape {
        Shape::Circle(r) => println!("this is circle with radius {}", r),
        Shape::Rectangle(h,b ) => println!("this is rectangle with H and B = {} and {}", h, b),
        Shape::Triangle(a, b,c ) => println!("this is triangle sides are {} {} {}", a, b, c)
    };
    
}
