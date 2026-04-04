pub mod calculator;
mod gui;
mod utils;

use crate::gui::{handle_calculate_btn_pressed, handle_window_size, init_gui};

// =================================================================================================
// This is the main function, the starting point of the entire program

fn main() {
    let widgets = init_gui();
    handle_window_size(widgets.clone());
    handle_calculate_btn_pressed(widgets.clone());

    widgets
        .calculator_window
        .run()
        .expect("Run the Calculator Window");
}
