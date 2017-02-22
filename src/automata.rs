
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
    outputs: Vec<Port>
}

/// Describe action and how to trigger it
#[derive(Debug, PartialEq)]
pub struct Action {
    name: String,
    /// Set given value on each port to change state
    inputs: Vec<Port>
}

/// Possible transmission between states with action which triggers this transition.
#[derive(Debug, PartialEq)]
pub struct Transition {
    src: u32,
    action: u32,
    dest: u32
}

#[derive(Debug, PartialEq)]
pub struct Port {
    name: String,
    port: u32,
    value: bool
}


impl System {
    /// The system will only be constructed if the parameters are correct.
    pub fn new(states: Vec<State>) -> Result<System, String> {
        Ok(System {
            states: states,
            actions: vec![],
            transitions: vec![],
            init_state: 0
        })
    }

    pub fn init_state(&self) -> &State {
        &self.states[self.init_state]
    }

//    pub fn find_transition(&self, state: &State) -> Option<&Transition> {
//        None
//    }
}

impl State {
    pub fn new(name: String, outputs: Vec<Port>) -> State {
        State { name: name, outputs: outputs}
    }
}

impl Port {
    pub fn new(name: String, port: u32, value: bool) -> Port {
        Port { name: name, port: port, value: value}
    }
}

