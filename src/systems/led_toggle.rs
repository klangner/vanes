/// Hardcoded system
/// This system consist of single state A
/// State A:
///   - Expects high value on given pin all the time


use hal::probes::{Probe};
use automata::{State, System};


// Temporary function. This sould be build from the script
pub fn build_system<'a>() -> System<'a> {

    let led1 = Probe::logic("Led 1".to_string(), 66, true);
    let states = vec![State::new("Led on".to_string(), vec![led1])];

    System::new(states, vec![])
        .expect("Can't construct system: led toggle")
}

