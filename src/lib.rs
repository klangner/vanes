
// IO Probe
enum Probe {
    Binary {port: i32, pin: i32},
    Freq {port: i32, pin: i32}
}

// Board state
struct State {
    led1 : bool
}

struct Model {
    states: Vec<State>,
    transitions: Vec<(i32, i32)>
}

// List of probes
static PROBES: Vec<Probe> = vec![Binary { port: 1, pin: 3}];

// Model board under test behaviour
static MODEL: Model = Model {
    states : State { led1: true },
    transitions: vec![]
};

// Measure state on board IO
fn measure_state(probes: &Vec<Probe>) -> State {

}

// Check if this state change is valid according to the model
fn assert_transition(state1: &State, state2: &State, model: &Model) -> bool {

}