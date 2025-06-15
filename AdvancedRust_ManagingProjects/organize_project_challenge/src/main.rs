///this should be main file only/////

mod shapes;
fn main() {
    let rect = shapes::Rectangle::new(1.0, 2.0);
    let area = rect.get_features(shapes::Feature::Area);
    println!("area is {}", area);

    let circle = shapes::Circle::new(3.0);
    let perimeter = circle.get_features(shapes::Feature::Perimeter);
    println!("perimenter is {} ", perimeter);
} 