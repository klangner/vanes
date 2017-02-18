
/// System Transition definition
/// initial and end state are index value to states vector.
#[derive(Debug, PartialEq)]
pub struct System {

    states: Vec<State>,
    actions: Vec<Action>,
    transitions: Vec<Transition>,
    init_state: u32,
    end_states: Vec<u32>
}

impl System {
    /// Empty system
    pub fn empty() -> System {
        System {
            states: vec![],
            actions: vec![],
            transitions: vec![],
            init_state: 0,
            end_states: vec![1]
        }
    }

    pub fn init_state(&self) -> Option<State> {
        None
    }
}

/// Describe state of the controller
#[derive(Debug, PartialEq)]
pub struct State {
    name: String,
    /// Expected values in this state on the given ports
    outputs: Vec<PortValue>
}

/// Describe action and how to trigger it
#[derive(Debug, PartialEq)]
pub struct Action {
    name: String,
    /// Set given value on each port to change state
    inputs: Vec<PortValue>
}

/// Possible transmission between states with action which triggers this transition.
#[derive(Debug, PartialEq)]
pub struct Transition {
    src: u32,
    action: u32,
    dest: u32
}

#[derive(Debug, PartialEq)]
pub struct PortValue {
    port: u32,
    value: bool
}
