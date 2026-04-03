mod roots;
mod extrema;
mod inflection_points;
mod utils;
mod intersection_points;
mod function_calculator;

use fltk::{app, button, input, prelude::*, window};
use fltk::button::CheckButton;
use fltk::enums::{Align, Event, Font};
use fltk::input::FloatInput;
use fltk::input::Input;
use fltk::{enums::FrameType, frame::Frame, image::PngImage, prelude::*};
use crate::function_calculator::function_calculator;
use crate::utils::*;

// =================================================================================================
// This is the main function, the starting point of the entire program

fn main() {

    // create app
    let calculator_window = app::App::default();
    // create app window
    let mut win = window::Window::default().with_size(800, 600).with_label("Function Calculator");
    win.make_resizable(true);

    // create the user Input fields
    let mut input_fields = Frame::new(0, 0, win.w(), win.h(), "");
    input_fields.set_frame(FrameType::BorderBox);
    input_fields.set_color(fltk::enums::Color::Light1);

    let font_size = (win.h() + win.w()) / 100;

    let v_rel_10 = win.h() / 60;
    let v_rel_20 = win.h() / 30;
    let v_rel_30 = win.h() / 20;

    let h_rel_10 = win.h() / 80;
    let h_rel_30 = win.h() / 20;
    let h_rel_80 = win.h() / 10;


    // positioning for function Inputs
    let input_parent_w = input_fields.w();
    let input_parent_h = input_fields.h();
    let function_input_w = win.w() - win.w() / 4;

        // create function A inputs
    let mut function_a_input = Input::default()
        .with_size(function_input_w, input_parent_h / 20)
        .with_pos((input_parent_w - function_input_w) / 2, input_parent_h / 30)
        .with_label("Function A (a_1, n_1, a_2, n_2...):");
    function_a_input.set_label_size(font_size);
    function_a_input.set_text_size(font_size);
    function_a_input.set_align(Align::Top | Align::Center); // Align label at the center above the input field
    function_a_input.set_value("f(x) = x^2"); // default function

        // create function B inputs
    let mut function_b_input = Input::default()
        .with_size(function_input_w, input_parent_h / 20)
        .below_of(&function_a_input, input_parent_h / 30)
        .with_label("Function B (a_1, n_1, a_2, n_2...):");
    function_b_input.set_label_size(font_size);
    function_b_input.set_text_size(font_size);
    function_b_input.set_align(Align::Top | Align::Center); // Align label at the center above the input field
    function_b_input.set_value("f(x) = 2x"); // default function
    
        // create newton interval inputs
    let mut newton_interval_input = Input::default()
        .with_size(function_input_w / 2, input_parent_h / 20)
        .with_pos((input_parent_w - function_input_w / 2) / 2, (input_parent_h / 20 * 2) + (input_parent_h / 30 * 3))
        .with_label("Interval for newton method (inclusive):");
    newton_interval_input.set_label_size(font_size);
    newton_interval_input.set_text_size(font_size);
    newton_interval_input.set_align(Align::Top | Align::Center);
    newton_interval_input.set_value("-1000, 1000");

        // positioning for bool inputs
    let bool_inputs_y = newton_interval_input.y() + newton_interval_input.height() + v_rel_20;
    let bool_inputs_w = win.w() / 3;
    let ck_btn_x = (input_parent_w - bool_inputs_w) / 2;
    
    // create check buttons for the user to select which calculations they want to perform
        // calculate root check button
    let mut root_ck_btn = CheckButton::default()
        .with_pos(ck_btn_x, bool_inputs_y)
        .with_size(bool_inputs_w, v_rel_30)
        .with_label("Calculate Roots");
    root_ck_btn.set_label_size(font_size);
    root_ck_btn.set_value(true); // default to calculating roots

        // calculate extrema check button
    let mut extrema_ck_btn = CheckButton::default()
        .with_size(bool_inputs_w, v_rel_30)
        .below_of(&root_ck_btn, v_rel_10)
        .with_label("Calculate Extrema");
    extrema_ck_btn.set_label_size(font_size);
    extrema_ck_btn.set_value(true); // default to calculating extrema

        // calculate inflection points check button
    let mut inflection_points_ck_btn = CheckButton::default()
        .with_size(bool_inputs_w, v_rel_30)
        .below_of(&extrema_ck_btn, v_rel_10)
        .with_label("Calculate Inflection Points");
    inflection_points_ck_btn.set_label_size(font_size);
    inflection_points_ck_btn.set_value(true); // default to calculating inflection points
    
        // calculate Y-value check button
    let mut y_value_ck_btn = CheckButton::default()
        .with_size(bool_inputs_w, v_rel_30)
        .below_of(&inflection_points_ck_btn, v_rel_10)
        .with_label("Calculate y-value at x:");
    y_value_ck_btn.set_label_size(font_size);
    y_value_ck_btn.set_value(true); // default to calculating y_value

        // X-value selector right below the Y-value check button
    let mut x_value_selector = FloatInput::default()
        .with_size(h_rel_80, v_rel_30)
        .below_of(&y_value_ck_btn, v_rel_10 / 2);
    x_value_selector.set_label_size(font_size);
    x_value_selector.set_text_size(font_size);
    x_value_selector.set_pos(ck_btn_x + h_rel_30, x_value_selector.y()); // slightly indented
    x_value_selector.set_value("0"); // default to calculating y_value at x = 0

        // calculate intersection points check button
    let mut intersection_points_ck_btn = CheckButton::default()
        .with_size(bool_inputs_w, v_rel_30)
        .below_of(&x_value_selector, v_rel_10)
        .with_label("Calculate Intersection Points");
    intersection_points_ck_btn.set_label_size(font_size);
    intersection_points_ck_btn.set_pos(ck_btn_x, intersection_points_ck_btn.y());
    intersection_points_ck_btn.set_value(true); // default to calculating intersection points

        // do plot function check button
    let mut plot_function_ck_btn = CheckButton::default()
        .with_size(bool_inputs_w, v_rel_30)
        .below_of(&intersection_points_ck_btn, v_rel_10)
        .with_label("Plot Function");
    plot_function_ck_btn.set_label_size(font_size);
    plot_function_ck_btn.set_value(false); // default to not plot the function

    // calculate button
    let mut calculate_btn = button::Button::default()
        .with_size(h_rel_80, v_rel_30)
        .with_pos((input_parent_w - h_rel_80) / 2 - h_rel_10, plot_function_ck_btn.y() + plot_function_ck_btn.h() + v_rel_20)
        .with_label("calculate");
    calculate_btn.set_label_size(font_size);

    // pre-create the plot frame (hidden initially)
    let mut plot_frame = Frame::new(win.h() / 2, 0, win.h() / 2, win.h(), "");
    plot_frame.set_frame(fltk::enums::FrameType::BorderBox);
    plot_frame.set_color(fltk::enums::Color::White);
    plot_frame.hide();
    
    win.end();
    win.show();
    
    // Clone for the resize handler
    let mut function_a_input_r = function_a_input.clone();
    let mut function_b_input_r = function_b_input.clone();
    let mut newton_interval_input_r = newton_interval_input.clone();
    let mut root_ck_btn_r = root_ck_btn.clone();
    let mut extrema_ck_btn_r = extrema_ck_btn.clone();
    let mut inflection_points_ck_btn_r = inflection_points_ck_btn.clone();
    let mut y_value_ck_btn_r = y_value_ck_btn.clone();
    let mut x_value_selector_r = x_value_selector.clone();
    let mut intersection_points_ck_btn_r = intersection_points_ck_btn.clone();
    let mut plot_function_ck_btn_r = plot_function_ck_btn.clone();
    let mut calculate_btn_r = calculate_btn.clone();

    win.handle(move |w, event| {
        match event {
            fltk::enums::Event::Resize => {
                let new_font_size = (input_fields.h() + input_fields.w()) / 100;
                function_a_input_r.set_label_size(new_font_size);
                function_a_input_r.set_text_size(new_font_size);

                function_b_input_r.set_label_size(new_font_size);
                function_b_input_r.set_text_size(new_font_size);

                newton_interval_input_r.set_label_size(new_font_size);
                newton_interval_input_r.set_text_size(new_font_size);

                root_ck_btn_r.set_label_size(new_font_size);

                extrema_ck_btn_r.set_label_size(new_font_size);

                inflection_points_ck_btn_r.set_label_size(new_font_size);

                y_value_ck_btn_r.set_label_size(new_font_size);

                x_value_selector_r.set_label_size(new_font_size);
                x_value_selector_r.set_text_size(new_font_size);

                intersection_points_ck_btn_r.set_label_size(new_font_size);

                plot_function_ck_btn_r.set_label_size(new_font_size);

                calculate_btn_r.set_label_size(new_font_size);
                true
            }
            _ => false,
        }
    });

    // Clone for the calculate button
    let mut function_a_input_c = function_a_input.clone();
    let mut function_b_input_c = function_b_input.clone();
    let mut newton_interval_input_c = newton_interval_input.clone();
    let mut root_ck_btn_c = root_ck_btn.clone();
    let mut extrema_ck_btn_c = extrema_ck_btn.clone();
    let mut inflection_points_ck_btn_c = inflection_points_ck_btn.clone();
    let mut y_value_ck_btn_c = y_value_ck_btn.clone();
    let mut x_value_selector_c = x_value_selector.clone();
    let mut intersection_points_ck_btn_c = intersection_points_ck_btn.clone();
    let mut plot_function_ck_btn_c = plot_function_ck_btn.clone();
    let mut plot_frame_c = plot_frame.clone();
    let mut win_c = win.clone();

    calculate_btn.set_callback(move |_| {
        let user_function_a = parse_polynomial(&function_a_input_c.value());
        let user_function_b = parse_polynomial(&function_b_input_c.value());
        let mut newton_interval = (-1000.0, 1000.0);
        if let Some((min, max)) = parse_interval(&newton_interval_input_c.value()) {
            newton_interval = (min, max);
        }
        let calculate_root = root_ck_btn_c.value();
        let calculate_extrema = extrema_ck_btn_c.value();
        let calculate_inflection_points = inflection_points_ck_btn_c.value();
        let calculate_y_value = y_value_ck_btn_c.value();
        let x_value = x_value_selector_c.value().parse().unwrap_or(0.0);
        let calculate_intersection_points = intersection_points_ck_btn_c.value();
        let do_plot_function = plot_function_ck_btn_c.value();

        function_calculator(
            user_function_a,
            user_function_b,
            newton_interval,
            calculate_root,
            calculate_extrema,
            calculate_inflection_points,
            calculate_y_value,
            x_value,
            calculate_intersection_points,
            do_plot_function,
        );

        if do_plot_function {
            win_c.set_size(win.w()*2, win_c.h());
            if let Ok(plotted_function) = PngImage::load("output.png") {
                plot_frame_c.set_size(win_c.w() / 2, win_c.h());
                plot_frame_c.set_pos(win_c.w() / 2, 0);
                plot_frame_c.set_image(Some(plotted_function));
            }
            plot_frame_c.show();
            win_c.redraw();
        } else {
            win_c.set_size(900, win_c.h());
            plot_frame_c.hide();
            win_c.redraw();
        }
    });

    calculator_window.run().unwrap();
}