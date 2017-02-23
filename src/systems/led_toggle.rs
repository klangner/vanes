/// Hardcoded system
/// This system consist of single state A
/// State A:
///   - Expects high value on given pin all the time


use hal::probes::{Probe};
use hal::drivers::{Driver};
use automata::{Action, State, Transition, System};


// Temporary function. This should be build from the script
pub fn build_system<'a>() -> System<'a> {

    let led1 = Probe::logic("Led 1".to_string(), 66, true);
    let button = Driver::button("Button down".to_string(), 68, false);
    let state_a = State::new("Led on".to_string(), vec![led1]);
    let action = Action { name: "Press button".to_string(), drivers: vec![button]};
    //let transition = Transition { src: &state_a, action: action, dest: &state_a};

    System::new(vec![state_a], vec![])
        .expect("Can't construct system: led toggle")
}

