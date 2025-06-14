#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triamgle(f64, f64, f64)
}

impl Shape {
    fn get_perimeter(&self) -> f64 {
        match self {
            Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
            Shape::Rectangle(l, b) => 2.0 * l + 2.0 * b,
            Shape::Triamgle(a,b ,c ) => a + b + c
        }
    }
}

fn main() {
    let my_shape = Shape::Rectangle(1.2, 3.5);
    println!("my_shape is {:?}", my_shape);

    let perimeter = my_shape.get_perimeter();
    println!("perimeter is {} ", perimeter);
   
}
