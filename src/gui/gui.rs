use fltk::button::{Button, CheckButton};
use fltk::enums::Align;
use fltk::input::FloatInput;
use fltk::input::Input;
use fltk::output::Output;
use fltk::window::Window;
use fltk::{app, button, prelude::*, window};
use fltk::{enums::FrameType, frame::Frame, prelude::*};

#[derive(Debug, Clone)]
pub struct Widgets {
    pub(crate) function_a_input: Input,
    pub(crate) function_b_input: Input,
    pub(crate) newton_interval_input: Input,
    pub(crate) root_ck_btn: CheckButton,
    pub(crate) extrema_ck_btn: CheckButton,
    pub(crate) inflection_points_ck_btn: CheckButton,
    pub(crate) y_value_ck_btn: CheckButton,
    pub(crate) x_value_selector: FloatInput,
    pub(crate) intersection_points_ck_btn: CheckButton,
    pub(crate) plot_function_ck_btn: CheckButton,
    pub(crate) calculate_btn: button::Button,
    pub(crate) root_output: Output,
    pub(crate) plot_frame: Frame,
    pub(crate) calculator_window: app::App,
    pub(crate) win: Window,
    pub(crate) input_fields: Frame,
}

#[derive(Debug, Clone)]
pub struct Positioning {
    font_size: i32,
    input_parent_w: i32,
    input_parent_h: i32,
    function_input_w: i32,
    bool_inputs_y: i32,
    bool_inputs_w: i32,
    ck_btn_x: i32,
    output_width: i32,
    output_height: i32,
}

#[derive(Debug, Clone)]
pub struct RelVal {
    v_rel_10: i32,
    v_rel_20: i32,
    v_rel_30: i32,
    h_rel_10: i32,
    h_rel_30: i32,
    h_rel_80: i32,
}

fn function_a_input(positioning: &Positioning) -> Input {
    let mut function_a_input = Input::default()
        .with_size(
            positioning.function_input_w,
            positioning.input_parent_h / 20,
        )
        .with_pos(
            (positioning.input_parent_w - positioning.function_input_w) / 2,
            positioning.input_parent_h / 30,
        )
        .with_label("Function A:");
    function_a_input.set_label_size(positioning.font_size);
    function_a_input.set_text_size(positioning.font_size);
    function_a_input.set_align(Align::Top | Align::Center); // Align label at the center above the input field
    function_a_input.set_value("f(x) = x^2"); // default function

    function_a_input
}

fn function_b_input(positioning: &Positioning, function_a_input: &Input) -> Input {
    let mut function_b_input = Input::default()
        .with_size(
            positioning.function_input_w,
            positioning.input_parent_h / 20,
        )
        .below_of(function_a_input, positioning.input_parent_h / 30)
        .with_label("Function B:");
    function_b_input.set_label_size(positioning.font_size);
    function_b_input.set_text_size(positioning.font_size);
    function_b_input.set_align(Align::Top | Align::Center); // Align label at the center above the input field
    function_b_input.set_value("f(x) = 2x"); // default function

    function_b_input
}

fn newton_interval_input(positioning: &Positioning, function_b_input: &Input) -> Input {
    let mut newton_interval_input = Input::default()
        .with_size(
            positioning.function_input_w / 2,
            positioning.input_parent_h / 20,
        )
        .below_of(function_b_input, positioning.input_parent_h / 30)
        .with_label("Interval for newton method (inclusive):");
    newton_interval_input.set_label_size(positioning.font_size);
    newton_interval_input.set_text_size(positioning.font_size);
    newton_interval_input.set_align(Align::Top | Align::Center);
    newton_interval_input.set_value("-1000, 1000");

    newton_interval_input
}

fn root_ck_btn(positioning: &Positioning, rel_val: &RelVal) -> CheckButton {
    let mut root_ck_btn = CheckButton::default()
        .with_pos(positioning.ck_btn_x, positioning.bool_inputs_y)
        .with_size(positioning.bool_inputs_w, rel_val.v_rel_30)
        .with_label("Calculate Roots");
    root_ck_btn.set_label_size(positioning.font_size);
    root_ck_btn.set_value(true); // default to calculating roots

    root_ck_btn
}

fn extrema_ck_btn(
    positioning: &Positioning,
    rel_val: &RelVal,
    root_ck_btn: &CheckButton,
) -> CheckButton {
    let mut extrema_ck_btn = CheckButton::default()
        .with_size(positioning.bool_inputs_w, rel_val.v_rel_30)
        .below_of(root_ck_btn, rel_val.v_rel_10)
        .with_label("Calculate Extrema");
    extrema_ck_btn.set_label_size(positioning.font_size);
    extrema_ck_btn.set_value(true); // default to calculating extrema

    extrema_ck_btn
}

fn inflection_points_ck_btn(
    positioning: &Positioning,
    rel_val: &RelVal,
    extrema_ck_btn: &CheckButton,
) -> CheckButton {
    let mut inflection_points_ck_btn = CheckButton::default()
        .with_size(positioning.bool_inputs_w, rel_val.v_rel_30)
        .below_of(extrema_ck_btn, rel_val.v_rel_10)
        .with_label("Calculate Inflection Points");
    inflection_points_ck_btn.set_label_size(positioning.font_size);
    inflection_points_ck_btn.set_value(true); // default to calculating inflection points

    inflection_points_ck_btn
}

fn y_value_ck_btn(
    positioning: &Positioning,
    rel_val: &RelVal,
    inflection_points_ck_btn: &CheckButton,
) -> CheckButton {
    let mut y_value_ck_btn = CheckButton::default()
        .with_size(positioning.bool_inputs_w, rel_val.v_rel_30)
        .below_of(inflection_points_ck_btn, rel_val.v_rel_10)
        .with_label("Calculate y-value at x:");
    y_value_ck_btn.set_label_size(positioning.font_size);
    y_value_ck_btn.set_value(true);

    y_value_ck_btn
}

fn x_value_selector(
    positioning: &Positioning,
    rel_val: &RelVal,
    y_value_ck_btn: &CheckButton,
) -> FloatInput {
    let mut x_value_selector = FloatInput::default()
        .with_size(rel_val.h_rel_80, rel_val.v_rel_30)
        .below_of(y_value_ck_btn, rel_val.v_rel_10 / 2);
    x_value_selector.set_label_size(positioning.font_size);
    x_value_selector.set_text_size(positioning.font_size);
    x_value_selector.set_pos(
        positioning.ck_btn_x + rel_val.h_rel_30,
        x_value_selector.y(),
    );
    x_value_selector.set_value("0");

    x_value_selector
}

fn intersection_pints_ck_btn(
    positioning: &Positioning,
    rel_val: &RelVal,
    x_value_selector: &FloatInput,
) -> CheckButton {
    let mut intersection_pints_ck_btn = CheckButton::default()
        .with_size(positioning.bool_inputs_w, rel_val.v_rel_30)
        .below_of(x_value_selector, rel_val.v_rel_10)
        .with_label("Calculate Intersection Points");
    intersection_pints_ck_btn.set_label_size(positioning.font_size);
    intersection_pints_ck_btn.set_pos(positioning.ck_btn_x, intersection_pints_ck_btn.y());
    intersection_pints_ck_btn.set_value(true);

    intersection_pints_ck_btn
}

fn plot_function_ck_btn(
    positioning: &Positioning,
    rel_val: &RelVal,
    intersection_pints_ck_btn: &CheckButton,
) -> CheckButton {
    let mut plot_function_ck_btn = CheckButton::default()
        .with_size(positioning.bool_inputs_w, rel_val.v_rel_30)
        .below_of(intersection_pints_ck_btn, rel_val.v_rel_10)
        .with_label("Plot Function");
    plot_function_ck_btn.set_label_size(positioning.font_size);
    plot_function_ck_btn.set_value(false);

    plot_function_ck_btn
}

fn calculate_btn(
    positioning: &Positioning,
    rel_val: &RelVal,
    plot_function_ck_btn: &CheckButton,
) -> Button {
    let mut calculate_btn = Button::default()
        .with_size(rel_val.h_rel_80, rel_val.v_rel_30)
        .with_pos(
            (positioning.input_parent_w - rel_val.h_rel_80) / 2 - rel_val.h_rel_10,
            plot_function_ck_btn.y() + plot_function_ck_btn.h() + rel_val.v_rel_20,
        )
        .with_label("calculate");
    calculate_btn.set_label_size(positioning.font_size);

    calculate_btn
}

fn root_output(output_width: &i32, output_height: &i32) -> Output {
    let mut output: Output = Output::default()
        .with_size(0, 0)
        .with_label("Roots: ")
        .with_align(fltk::enums::Align::Left);
    output.hide();

    output
}

pub fn init_gui() -> Widgets {
    // create app
    let calculator_window = app::App::default();
    // create app window
    let mut win = window::Window::default()
        .with_size(800, 600)
        .with_label("Function Calculator");

    // create the user Input fields
    let mut input_fields = Frame::new(0, 0, win.w(), win.h(), "");
    input_fields.set_frame(FrameType::BorderBox);
    input_fields.set_color(fltk::enums::Color::Light1);

    let mut rel_val = RelVal {
        v_rel_10: win.h() / 60,
        v_rel_20: win.h() / 30,
        v_rel_30: win.h() / 20,

        h_rel_10: win.h() / 80,
        h_rel_30: win.h() / 20,
        h_rel_80: win.h() / 10,
    };

    let mut positioning = Positioning {
        font_size: (win.h() + win.w()) / 100,

        input_parent_w: input_fields.w(),
        input_parent_h: input_fields.h(),

        function_input_w: win.w() - win.w() / 4,

        bool_inputs_y: 0,
        bool_inputs_w: 0,

        ck_btn_x: 0,

        output_width: win.width() / 3 - rel_val.v_rel_30,
        output_height: win.h() / 10,
    };

    let function_a_input = function_a_input(&positioning);
    let function_b_input = function_b_input(&positioning, &function_a_input);
    let newton_interval_input = newton_interval_input(&positioning, &function_b_input);

    // positioning for bool inputs
    positioning.bool_inputs_y =
        newton_interval_input.y() + newton_interval_input.height() + rel_val.v_rel_20;
    positioning.bool_inputs_w = win.w() / 3;
    positioning.ck_btn_x = (positioning.input_parent_w - positioning.bool_inputs_w) / 2;

    let root_ck_btn = root_ck_btn(&positioning, &rel_val);
    let extrema_ck_btn = extrema_ck_btn(&positioning, &rel_val, &root_ck_btn);
    let inflection_points_ck_btn =
        inflection_points_ck_btn(&positioning, &rel_val, &extrema_ck_btn);
    let y_value_ck_btn = y_value_ck_btn(&positioning, &rel_val, &inflection_points_ck_btn);
    let x_value_selector = x_value_selector(&positioning, &rel_val, &y_value_ck_btn);
    let intersection_points_ck_btn =
        intersection_pints_ck_btn(&positioning, &rel_val, &x_value_selector);
    let plot_function_ck_btn =
        plot_function_ck_btn(&positioning, &rel_val, &intersection_points_ck_btn);
    let calculate_btn = calculate_btn(&positioning, &rel_val, &plot_function_ck_btn);

    let root_output = root_output(&positioning.output_width, &positioning.output_height);

    // pre-create the plot frame (hidden initially)
    let mut plot_frame = Frame::new(win.w(), 0, win.w(), win.h(), "");
    plot_frame.set_frame(fltk::enums::FrameType::BorderBox);
    plot_frame.set_color(fltk::enums::Color::White);
    plot_frame.hide();

    win.end();
    win.show();

    let widgets = Widgets {
        function_a_input,
        function_b_input,
        newton_interval_input,
        root_ck_btn,
        extrema_ck_btn,
        inflection_points_ck_btn,
        y_value_ck_btn,
        x_value_selector,
        intersection_points_ck_btn,
        plot_function_ck_btn,
        calculate_btn,
        root_output,
        plot_frame,
        calculator_window,
        win,
        input_fields,
    };

    widgets
}
