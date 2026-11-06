// instead of validation, parse it into validated newtypes with 'TryFrom'
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Port(u16);

impl Port {
    pub fn get(&self) -> u16 {
        self.0
    }
}

impl FromStr for Port {
    type Err = PortError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n: u16 = s.parse().map_err(|_| PortError::InvalidFormat)?;
        Port::try_from(n)
    }
}

impl TryFrom<u16> for Port {
    type Error = PortError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(PortError::Zero)
        } else {
            Ok(Port(value))
        }
    }
}

#[derive(Debug)]
pub enum PortError {
    Zero,
    InvalidFormat,
}

impl fmt::Display for PortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PortError::Zero => write!(f, "port must be none-zero"),
            PortError::InvalidFormat => write!(f, "invalid port format"),
        }
    }
}

impl std::error::Error for PortError {}

fn start_server(port: Port) {
    println!("Listening on port {}", port.get());
}

fn example() -> Result<(), Box<dyn std::error::Error>> {
    let port = Port::try_from(8080)?;
    start_server(port);

    let port2: Port = "8080".parse()?;
    start_server(port2);

    if let Err(e) = Port::try_from(0) {
        eprintln!("{e}");
    }

    Ok(())
}
