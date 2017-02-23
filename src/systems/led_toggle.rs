/// Hardcoded system
/// This system consist of single state A
/// State A:
///   - Expects high value on given pin all the time


use automata::{Port, State, System};


// state1 = "Led off"
// state2 = "Led on"
// action = "button pressed"
// transitions = [(state1, action, state2), (state2, action, state1)
pub fn build_system<'a>() -> System<'a> {

    let led1 = Port::new("Led 1".to_string(), 1234, true);
    let states = vec![State::new("Led on".to_string(), vec![led1])];

    System::new(states, vec![])
        .expect("Can't construct system: led toggle")
}

