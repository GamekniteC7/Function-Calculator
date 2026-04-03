mod roots;
mod extrema;
mod inflection_points;
mod utils;
mod intersection_points;
mod function_calculator;

use fltk::{app, button, input, prelude::*, window};
use fltk::button::CheckButton;
use fltk::enums::Align;
use fltk::input::FloatInput;
use fltk::input::Input;
use crate::function_calculator::function_calculator;
use crate::utils::*;

// =================================================================================================
// This is the main function, the starting point of the entire program

fn main() {

    // Here the user may set the desired interval
    let newton_interval: (f64, f64) = (-100.0, 100.0);

    // ---------------------------------------------------------------------------------------------

    // create app
    let calculator_window = app::App::default();
    // create app window
    let mut win = window::Window::default().with_size(800, 600).with_label("Function Calculator");

    // create the user Input fields

        // positioning for function Inputs
    let parent_w = win.width();
    let function_input_w = 600;

        // create function inputs
    let mut function_a_input = Input::default().with_size(function_input_w, 30).with_pos((parent_w - function_input_w) / 2, 20).with_label("Function A (a_1, n_1, a_2, n_2...):");
    function_a_input.set_align(Align::Top | Align::Center); // Align label at the center above the input field
    function_a_input.set_value("f(x) = x^2"); // default function
    let mut function_b_input = Input::default().with_size(function_input_w, 30).below_of(&function_a_input, 20).with_label("Function B (a_1, n_1, a_2, n_2...):");
    function_b_input.set_align(Align::Top | Align::Center); // Align label at the center above the input field
    function_b_input.set_value("f(x) = 2x"); // default function

        // positioning for bool inputs
    let bool_inputs_y = function_b_input.y() + function_b_input.height() + 20;
    let bool_inputs_w = 280;
    let ck_btn_x = (parent_w - bool_inputs_w) / 2;
    
        // create bool inputs
    let mut root_ck_btn = CheckButton::default().with_pos(ck_btn_x, bool_inputs_y).with_size(bool_inputs_w, 30).with_label("Calculate Roots");
    root_ck_btn.set_value(true); // default to calculating roots
    let mut extrema_ck_btn = CheckButton::default().with_size(bool_inputs_w, 30).below_of(&root_ck_btn, 10).with_label("Calculate Extrema");
    extrema_ck_btn.set_value(true); // default to calculating extrema
    let mut inflection_points_ck_btn = CheckButton::default().with_size(bool_inputs_w, 30).below_of(&extrema_ck_btn, 10).with_label("Calculate Inflection Points");
    inflection_points_ck_btn.set_value(true); // default to calculating inflection points
    
        // Y-value check button
    let mut y_value_ck_btn = CheckButton::default().with_size(bool_inputs_w, 30).below_of(&inflection_points_ck_btn, 10).with_label("Calculate y-value at x:");
    y_value_ck_btn.set_value(true); // default to calculating y_value
        // X-value selector right below the Y-value check button
    let mut x_value_selector = FloatInput::default().with_size(80, 30).below_of(&y_value_ck_btn, 5);
    x_value_selector.set_pos(ck_btn_x + 30, x_value_selector.y()); // slightly indented
    x_value_selector.set_value("0"); // default to calculating y_value at x = 0

    let mut intersection_points_ck_btn = CheckButton::default().with_size(bool_inputs_w, 30).below_of(&x_value_selector, 10);
    intersection_points_ck_btn.set_pos(ck_btn_x, intersection_points_ck_btn.y());
    intersection_points_ck_btn.set_label("Calculate Intersection Points");
    intersection_points_ck_btn.set_value(true); // default to calculating intersection points

    let mut plot_function_ck_btn = CheckButton::default().with_size(bool_inputs_w, 30).below_of(&intersection_points_ck_btn, 10).with_label("Plot Function");
    plot_function_ck_btn.set_value(false); // default to not plot the function

    // create run button for the program
        // positioning for calculate button
    let calculate_btn_w = 80;
    let mut calculate_btn = button::Button::default().with_size(80, 30).with_pos((parent_w - calculate_btn_w) / 2 - 10, plot_function_ck_btn.y() + plot_function_ck_btn.h() + 20).with_label("calculate");


    win.end();
    win.show();

    calculate_btn.set_callback(move |_| {

        // get user Inputs from the UI
        let user_function_a = parse_polynomial(&function_a_input.value());
        let user_function_b = parse_polynomial(&function_b_input.value());
        let calculate_root = root_ck_btn.value();
        let calculate_extrema = extrema_ck_btn.value();
        let calculate_inflection_points = inflection_points_ck_btn.value();
        let calculate_y_value = y_value_ck_btn.value();
        let x_value = x_value_selector.value().parse().unwrap_or(0.0);
        let calculate_intersection_points = intersection_points_ck_btn.value();
        let do_plot_function = plot_function_ck_btn.value();

        println!("Function A variables: {:?}", user_function_a);

        function_calculator(
            user_function_a.clone(),
            user_function_b.clone(),
            newton_interval.clone(),
            calculate_root.clone(),
            calculate_extrema.clone(),
            calculate_inflection_points.clone(),
            calculate_y_value.clone(),
            x_value.clone(),
            calculate_intersection_points.clone(),
            do_plot_function.clone()
        );
    });

    calculator_window.run().unwrap();
}