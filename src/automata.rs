
/// System Transition definition
/// initial and end state are index value to states vector.
#[derive(Debug, PartialEq)]
pub struct System {
    states: Vec<State>,
    actions: Vec<Action>,
    transitions: Vec<Transition>,
    init_state: usize
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


impl System {
    /// Empty system
    pub fn single_state(state: State) -> System {
        System {
            states: vec![state],
            actions: vec![],
            transitions: vec![],
            init_state: 0
        }
    }

    pub fn init_state(&self) -> &State {
        &self.states[self.init_state]
    }

//    pub fn find_transition(&self, state: &State) -> Option<&Transition> {
//        None
//    }
}

impl State {
    pub fn new(name: String, outputs: Vec<PortValue>) -> State {
        State { name: name, outputs: outputs}
    }
}

impl PortValue {
    pub fn new(port: u32, value: bool) -> PortValue {
        PortValue { port: port, value: value}
    }
}

