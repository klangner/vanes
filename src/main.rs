#![feature(lang_items)]
#![no_std]
#![no_main]

extern crate sam3x;

use sam3x::*;
use sam3x::pio;
use sam3x::rtt;

mod automata;
mod led_toggle_system;


#[link_section=".vectors"]
pub static VECTOR_TABLE: VectorTable =
    VectorTable {
        reset_handler : start,
        exceptions    : [0; 14],
    };


/// Main function connected to the reset handler
/// Arduino Led is connected to the controller B, line 27
fn start() -> ! {
    let system = build_system();
    let mut state = system.init_state;

    loop {
        let t = find_transition(system, state);
        execute_action(t.action);
        wait(1);
        state = t.end_state;
        if check_state(state) {
            error();
        }
        wait(100);
    }
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! { loop {} }

// This function is needed by arm exception handling routines.
#[no_mangle] pub extern fn __aeabi_unwind_cpp_pr0() { loop {} }
#[no_mangle] pub extern fn __aeabi_unwind_cpp_pr1() { loop {} }
