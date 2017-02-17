
/// System Transition definition
/// initial and end state are index value to states vector.
struct System {

    states: Vec<State>,
    actions: Vec<Action>,
    transitions: Vec<Transition>,
    init_state: u32,
    end_states: Vec<u32>
}

/// Describe state of the controller
struct State {
    name: str
    // Ports and their values
}

/// Describe action and how to trigger it
struct Action {
    name: str
    // What values and to which ports provide to change execute this action
}

/// Possible transmission between states with action which triggers this transition.
struct Transition {
    src: u32,
    action: u32,
    dest: u32
}
