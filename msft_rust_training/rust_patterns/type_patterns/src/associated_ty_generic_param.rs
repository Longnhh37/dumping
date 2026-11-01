// --- ASSOCIATED TYPE: One implementation per type ---
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    max: i32,
    current: i32,
}

impl Iterator for Counter {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

// --- GENERIC PARAMETER: Multiple implementations per type ---
trait Convert<T> {
    fn convert(&self) -> T;
}

impl Convert<f64> for i32 {
    fn convert(&self) -> f64 {
        *self as f64
    }
}
impl Convert<String> for i32 {
    fn convert(&self) -> String {
        self.to_string()
    }
}

trait Add<Rhs = Self> {
    type Output; // Associated type
    fn add(self, rhs: Rhs) -> Self::Output;
}

// Rhs is a generic parameter
struct Meters(f64);
struct Centimeters(f64);

impl Add<Meters> for Meters {
    type Output = Meters;
    fn add(self, rhs: Meters) -> Meters {
        Meters(self.0 + rhs.0)
    }
}
impl Add<Centimeters> for Meters {
    type Output = Meters;
    fn add(self, rhs: Centimeters) -> Meters {
        Meters(self.0 + rhs.0 / 100.0)
    }
}
