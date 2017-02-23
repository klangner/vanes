
use automata::{State, Action};


/// Check if all ports have values according to the state
pub fn check_state(_: &State) -> bool {
    false
}

pub fn execute_action(action: &Action) {
    println!("Execute action {:?}", action);
}