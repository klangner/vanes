
use sysfs_gpio::{Pin};


/// Driver simulates test board inputs
#[derive(Debug, PartialEq)]
pub enum Driver {
    /// Button is driver which can be in state LOW or HIGH
    Button { name: String, pin: Pin, value: bool }
}


impl Driver {
    pub fn button(name: String, port: u64, value: bool) -> Driver {
        let pin = Pin::new(port);
        pin.export().unwrap();
        Driver::Button {name: name, pin: pin, value: value}
    }
}