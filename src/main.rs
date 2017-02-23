
extern crate sysfs_gpio;

mod automata;
mod hal {
    pub mod drivers;
    pub mod probes;
}
mod systems{
    pub mod led_toggle;
}

use std::time::Duration;
use std::thread::{sleep};
use automata::{State};
use systems::led_toggle::{build_system};



/// Main function connected to the reset handler
/// Arduino Led is connected to the controller B, line 27
fn main() {
    let system = build_system();
    let ref mut state: &State = system.init_state();

    println!("Init state: {:?}", state);

    loop {
        // Lets give the system some time to enjoy its current state
        sleep(Duration::from_millis(500));
        // Are we still in the correct state?
        // The system shouldn't change the state itself.
        assert!(state.check(), "ERROR: System changed state itself. Expected {:?}", state);
        // What transition go from the current state?
        if let Some(t) = system.find_transition(state) {
            println!("Execute action: {:?}", &t.action);
            &t.action.execute();
            // Let wait till the signals will propagate
            sleep(Duration::from_millis(500));
            // And check if we are in desired state
            assert!(state.check(), "ERROR: System changed state itself. Expected {:?}", state);
            *state = t.dest;
            println!("Switched to state: {:?}", state);
        };
    }
}


//fn main() {
//    // Init button
//    let button = Pin::new(66);
//    button.export().unwrap();
//    button.set_direction(Direction::In).unwrap();
//    // Init led
//    let led = Pin::new(68);
//    led.export().unwrap();
//
//    loop {
//        let value: u8 = button.get_value().unwrap();
//        led.set_value(value).unwrap();
//        sleep(Duration::from_millis(100));
//    }
//}