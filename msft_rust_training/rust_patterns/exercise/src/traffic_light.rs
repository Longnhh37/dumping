// ===================================================================
// Typestate with type (state is type, checked when compiling)
// ===================================================================
use std::marker::PhantomData;

struct Red;
struct Green;
struct Yellow;

struct Light<S> {
    _state: PhantomData<S>,
}

impl Light<Red> {
    fn new() -> Self {
        Self {
            _state: PhantomData,
        }
    }
    fn next(self) -> Light<Yellow> {
        Light {
            _state: PhantomData,
        }
    }
}

impl Light<Yellow> {
    fn next(self) -> Light<Green> {
        Light {
            _state: PhantomData,
        }
    }
}

impl Light<Green> {
    fn next(self) -> Light<Red> {
        Light {
            _state: PhantomData,
        }
    }

    // only exists in the Green state
    fn drive(&self) {
        println!("go");
    }
}

// ===================================================================
// Typestate with enum (state is value, check during runtime
// ===================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Light2 {
    Red,
    Green,
    Yellow,
}

impl Light2 {
    fn next(self) -> Self {
        match self {
            Self::Red => Self::Yellow,
            Self::Yellow => Self::Green,
            Self::Green => Self::Red,
        }
    }

    // state is checked at runtime and return an error
    fn drive(self) -> Result<(), &'static str> {
        match self {
            Self::Green => {
                println!("go");
                Ok(())
            }
            _ => Err("light is not green"),
        }
    }
}

// ===================================================================
// Comparison
// ===================================================================
fn example() {
    let _light = Light::new(); // Light<Red>
    // light.drive(); // compile error: no method 'drive' for Light<Red>

    let _light = Light2::Red;
    // light.drive().unwrap() // compiles ok, crash at runtime
}
