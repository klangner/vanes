/// In the future this will be written as test script in custom language
/// But know the model is build in the code

use automata::System;


// state1 = "Led off"
// state2 = "Led on"
// action = "button pressed"
// transitions = [(state1, action, state2), (state2, action, state1)
pub fn build_system() -> System {
    System::empty()
}

