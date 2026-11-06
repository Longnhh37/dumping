mod private {
    // `pub` trait inside a private module: nameable only within this crate
    pub trait Sealed {}
}

// Closed set of shapes. Downstream crates cannot add new ones.
pub trait Shape: private::Sealed {
    fn area(&self) -> f64;

    // Safe to add new methods later: no downstream impl can break.
    fn name(&self) -> &'static str {
        "shape"
    }
}

pub struct Circle(pub f64);
pub struct Square(pub f64);

impl private::Sealed for Circle {}
impl private::Sealed for Square {}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.0 * self.0
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

// Users can still use `Shape` as a bound
pub fn total_area(shapes: &[&dyn Shape]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}
