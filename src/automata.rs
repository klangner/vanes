
use hal::probes::{Probe};
use hal::drivers::{Driver};


/// System Transition definition
/// initial and end state are index value to states vector.
#[derive(Debug, PartialEq)]
pub struct System<'a> {
    states: Vec<State>,
    transitions: Vec<Transition<'a>>
}

/// Describe state of the controller
#[derive(Debug, PartialEq)]
pub struct State {
    pub name: String,
    /// Expected values in this state on the given ports
    pub probes: Vec<Probe>
}

/// Describe action and how to trigger it
#[derive(Debug, PartialEq)]
pub struct Action {
    pub name: String,
    /// Set given value on each port to change state
    pub inputs: Vec<Driver>
}

/// Possible transmission between states with action which triggers this transition.
#[derive(Debug, PartialEq)]
pub struct Transition<'a> {
    pub src: &'a State,
    pub action: Action,
    pub dest: &'a State
}

impl<'a> System<'a> {
    /// The system will only be constructed if the parameters are correct.
    /// At least it needs one state.
    /// The first state on the list will be initial state.
    pub fn new(states: Vec<State>, transitions: Vec<Transition<'a>>)
        -> Result<System<'a>, &'static str>
    {
        if states.len() == 0 {
            Err("No states")
        } else {
            Ok(System {
                states: states,
                transitions: transitions,
            })
        }
    }

    pub fn init_state(&self) -> &State {
        &self.states[0]
    }

    pub fn find_transition(&self, state: &State) -> Option<&Transition> {
        self.transitions.iter().find(|t| t.src == state)
    }
}

impl State {
    /// Constructor for new state
    pub fn new(name: String, outputs: Vec<Probe>) -> State {
        State { name: name, probes: outputs}
    }

    /// Check if the probe values are equal to expected values
    pub fn check(&self) -> bool {
        self.probes.iter().fold(true, |acc, p| {
            let v = p.has_correct_value();
            if v == false {
                println!("Wrong value on probe: {:?}", p);
            };
            acc && v
        })
    }
}

impl Action {
    pub fn execute(&self) {
        println!("Execute action {:?}", self);
    }
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_without_states() {
        let system = System::new(vec![], vec![]);
        assert!(system.is_err());
    }
}