/*Define struct to represent Rectangle
Fields height :f64
       width  :f64
Methods get_area() return widht*height
        scale() scale rectangle height and width
AssociatedMetgod
        new(), input width and height and return rectangle

*/
#[derive(Clone, Copy)]
struct Rectangle{
    height :f64,
    width: f64
}

impl Rectangle {
    fn get_area(self) -> f64 {
        return self.height * self.width;
    }
    fn scale(&mut self, scale_factor:f64) {
        self.height = self.height * scale_factor;
        self.width = self.width * scale_factor;
    }

    fn new(newheight:f64, newwidth:f64) -> Rectangle {
        return  Rectangle { height: (newheight), width: (newwidth) };
    }
}
fn main() {
    let mut rect = Rectangle::new(1.2, 3.5);
    assert_eq!(rect.get_area(), 4.2);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.05);

    println!("test passed");
}
