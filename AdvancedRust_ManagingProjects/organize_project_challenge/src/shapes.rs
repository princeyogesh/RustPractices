use std::f64::consts::PI;

pub struct Circle {
    radius: f64
}

pub struct Rectangle {
    height : f64,
    width : f64
}

pub enum Feature {
    Area,
    Perimeter
}


impl Rectangle {
    pub fn new(height:f64, width:f64) -> Rectangle {
        Rectangle{height, width}
    }
    pub fn get_features(&self, feat: Feature) -> f64 {
        match feat {
            Feature::Area => self.height * self.width,
            Feature::Perimeter => 2.0 * (self.height + self.width)
        }
    }
}
impl Circle {
   pub fn new(radius:f64) ->Circle {
        Circle{radius}
    }

    pub fn get_features(&self, feat: Feature) -> f64 {
        match feat {
            Feature::Area => self.radius * self.radius * PI,
            Feature::Perimeter => 2.0 * PI * self.radius
        }
    }
}