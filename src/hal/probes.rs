
use sysfs_gpio::{Direction, Pin};


/// Probe is used to check values on tested board
#[derive(Debug, PartialEq)]
pub enum Probe {
    /// Logic probe can check binary values. Most often used in GPIO
    LogicProbe { name: String, pin: Pin, expected: bool }
}

impl Probe {

    /// Create logic probe
    pub fn logic(name: String, port: u64, expected: bool) -> Probe {
        let pin = Pin::new(port);
        pin.export().unwrap();
        pin.set_direction(Direction::In).unwrap();
        Probe::LogicProbe {name: name, pin: pin, expected: expected}
    }

    /// Check if probe has correct value
    pub fn has_correct_value(&self) -> bool {
        match self {
            &Probe::LogicProbe {ref pin, expected, ..} => read_logic(&pin) == expected
        }
    }
}

// Get value from GPIO
fn read_logic(pin: &Pin) -> bool {
    pin.get_value().unwrap() == 1
}
