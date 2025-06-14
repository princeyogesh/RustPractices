
/*
    Define data type with multple possible variants is Enum in rust
*/
#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triamgle(f64, f64, f64)
}

/*
    another use case for drawing programme
    enum Command {
        Clear,
        DrawLine(f64, f64),
        DrawShape(Shape)
    }
*/
fn main() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("myshape = {:?}", my_shape);
}
