/// Hardcoded system
/// This system consist of single state A
/// State A:
///   - Expects high value on given pin all the time


use automata::{PortValue, State, System};


// state1 = "Led off"
// state2 = "Led on"
// action = "button pressed"
// transitions = [(state1, action, state2), (state2, action, state1)
pub fn build_system() -> System {

    let output_port = PortValue::new(1234, true);
    let state_a = State::new("A".to_string(), vec![output_port]);
//    states: Vec<State>,
//    actions: Vec<Action>,
//    transitions: Vec<Transition>,
//    init_state: u32,
//    end_states: Vec<u32>

    System::single_state(state_a)
}

