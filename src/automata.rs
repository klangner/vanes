
/// System Transition definition
/// initial and end state are index value to states vector.
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
}

/// Describe state of the controller
struct State {
    name: str,
    /// Expected values in this state on the given ports
    outputs: Vec<PortValue>
}

/// Describe action and how to trigger it
struct Action {
    name: str,
    /// Set given value on each port to change state
    inputs: Vec<PortValue>
}

/// Possible transmission between states with action which triggers this transition.
struct Transition {
    src: u32,
    action: u32,
    dest: u32
}

struct PortValue {
    pin: BinaryPin,
    value: bool
}
