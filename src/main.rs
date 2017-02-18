
mod automata;
mod led_toggle_system;

use led_toggle_system::{build_system};



/// Main function connected to the reset handler
/// Arduino Led is connected to the controller B, line 27
fn main() {
    let system = build_system();
    let mut state = system.init_state();

    println!("Init state: {:?}", state);

    loop {
//        // Lets give the system some time to enjoy its current state
//        wait(100);
//        // Are we still in the correct state?
//        if check_state(state) {
//            error();
//        }
//        // What transition go from the current state?
//        let t = find_transition(system, state);
//        execute_action(t.action);
//        // Let wait till the signals will propagate
//        wait(1);
//        // And check if we are in desired state
//        state = t.end_state;
//        if check_state(state) {
//            error();
//        }
    }
}
