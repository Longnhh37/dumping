// Standard library marker traits:
// Send    — safe to transfer between threads
// Sync    — safe to share (&T) between threads
// Unpin   — safe to move after pinning
// Sized   — has a known size at compile time
// Copy    — can be duplicated with memcpy

// Your own marker trait:
/// Marker: this sensor has been factory-calibrated
trait Calibrated {}

struct RawSensor {
    reading: f64,
}
struct CalibratedSensor {
    reading: f64,
}

impl Calibrated for CalibratedSensor {}

// Only calibrated sensors can be used in production:
fn record_measurement<S: Calibrated>(sensor: &S) {
    // ...
}
// record_measurement(&RawSensor { reading: 0.0 }); // Compile error
// record_measurement(&CalibratedSensor { reading: 0.0 }); // compile ok
