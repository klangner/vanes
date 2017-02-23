
extern crate sysfs_gpio;

mod automata;
mod hal;
mod systems{
    pub mod led_toggle;
}

use std::time::Duration;
use std::thread;
use automata::{State};
use hal::{check_state, execute_action};
use systems::led_toggle::{build_system};



/// Main function connected to the reset handler
/// Arduino Led is connected to the controller B, line 27
fn main() {
    let system = build_system();
    let ref mut state: &State = system.init_state();

    println!("Init state: {:?}", state);

    loop {
        // Lets give the system some time to enjoy its current state
        thread::sleep(Duration::from_millis(500));
        // Are we still in the correct state?
        // The system shouldn't change the state itself.
        assert!(check_state(state), "ERROR: System changed state itself. Expected {:?}", state);
        // What transition go from the current state?
        if let Some(t) = system.find_transition(state) {
            execute_action(&t.action);
            // Let wait till the signals will propagate
            thread::sleep(Duration::from_millis(500));
            // And check if we are in desired state
            *state = t.dest;
        };
    }
}
