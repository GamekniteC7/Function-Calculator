use fltk::button::{Button, CheckButton};
use fltk::enums::Align;
use fltk::input::FloatInput;
use fltk::input::Input;
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
    pub(crate) plot_frame: Frame,
    pub(crate) calculator_window: app::App,
    pub(crate) win: Window,
    pub(crate) input_fields: Frame,
}

#[derive(Debug, Clone)]
pub struct Positioning {
    font_size: i32,
    v_rel_10: i32,
    v_rel_20: i32,
    v_rel_30: i32,
    h_rel_10: i32,
    h_rel_30: i32,
    h_rel_80: i32,
    input_parent_w: i32,
    input_parent_h: i32,
    function_input_w: i32,
    bool_inputs_y: i32,
    bool_inputs_w: i32,
    ck_btn_x: i32,
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

fn root_ck_btn(positioning: &Positioning) -> CheckButton {
    let mut root_ck_btn = CheckButton::default()
        .with_pos(positioning.ck_btn_x, positioning.bool_inputs_y)
        .with_size(positioning.bool_inputs_w, positioning.v_rel_30)
        .with_label("Calculate Roots");
    root_ck_btn.set_label_size(positioning.font_size);
    root_ck_btn.set_value(true); // default to calculating roots

    root_ck_btn
}

fn extrema_ck_btn(positioning: &Positioning, root_ck_btn: &CheckButton) -> CheckButton {
    let mut extrema_ck_btn = CheckButton::default()
        .with_size(positioning.bool_inputs_w, positioning.v_rel_30)
        .below_of(root_ck_btn, positioning.v_rel_10)
        .with_label("Calculate Extrema");
    extrema_ck_btn.set_label_size(positioning.font_size);
    extrema_ck_btn.set_value(true); // default to calculating extrema

    extrema_ck_btn
}

fn inflection_points_ck_btn(
    positioning: &Positioning,
    extrema_ck_btn: &CheckButton,
) -> CheckButton {
    let mut inflection_points_ck_btn = CheckButton::default()
        .with_size(positioning.bool_inputs_w, positioning.v_rel_30)
        .below_of(extrema_ck_btn, positioning.v_rel_10)
        .with_label("Calculate Inflection Points");
    inflection_points_ck_btn.set_label_size(positioning.font_size);
    inflection_points_ck_btn.set_value(true); // default to calculating inflection points

    inflection_points_ck_btn
}

fn y_value_ck_btn(
    positioning: &Positioning,
    inflection_points_ck_btn: &CheckButton,
) -> CheckButton {
    let mut y_value_ck_btn = CheckButton::default()
        .with_size(positioning.bool_inputs_w, positioning.v_rel_30)
        .below_of(inflection_points_ck_btn, positioning.v_rel_10)
        .with_label("Calculate y-value at x:");
    y_value_ck_btn.set_label_size(positioning.font_size);
    y_value_ck_btn.set_value(true);

    y_value_ck_btn
}

fn x_value_selector(positioning: &Positioning, y_value_ck_btn: &CheckButton) -> FloatInput {
    let mut x_value_selector = FloatInput::default()
        .with_size(positioning.h_rel_80, positioning.v_rel_30)
        .below_of(y_value_ck_btn, positioning.v_rel_10 / 2);
    x_value_selector.set_label_size(positioning.font_size);
    x_value_selector.set_text_size(positioning.font_size);
    x_value_selector.set_pos(
        positioning.ck_btn_x + positioning.h_rel_30,
        x_value_selector.y(),
    );
    x_value_selector.set_value("0");

    x_value_selector
}

fn intersection_pints_ck_btn(
    positioning: &Positioning,
    x_value_selector: &FloatInput,
) -> CheckButton {
    let mut intersection_pints_ck_btn = CheckButton::default()
        .with_size(positioning.bool_inputs_w, positioning.v_rel_30)
        .below_of(x_value_selector, positioning.v_rel_10)
        .with_label("Calculate Intersection Points");
    intersection_pints_ck_btn.set_label_size(positioning.font_size);
    intersection_pints_ck_btn.set_pos(positioning.ck_btn_x, intersection_pints_ck_btn.y());
    intersection_pints_ck_btn.set_value(true);

    intersection_pints_ck_btn
}

fn plot_function_ck_btn(
    positioning: &Positioning,
    intersection_pints_ck_btn: &CheckButton,
) -> CheckButton {
    let mut plot_function_ck_btn = CheckButton::default()
        .with_size(positioning.bool_inputs_w, positioning.v_rel_30)
        .below_of(intersection_pints_ck_btn, positioning.v_rel_10)
        .with_label("Plot Function");
    plot_function_ck_btn.set_label_size(positioning.font_size);
    plot_function_ck_btn.set_value(false);

    plot_function_ck_btn
}

fn calculate_btn(positioning: &Positioning, plot_function_ck_btn: &CheckButton) -> Button {
    let mut calculate_btn = Button::default()
        .with_size(positioning.h_rel_80, positioning.v_rel_30)
        .with_pos(
            (positioning.input_parent_w - positioning.h_rel_80) / 2 - positioning.h_rel_10,
            plot_function_ck_btn.y() + plot_function_ck_btn.h() + positioning.v_rel_20,
        )
        .with_label("calculate");
    calculate_btn.set_label_size(positioning.font_size);

    calculate_btn
}

pub fn init_gui() -> Widgets {
    // create app
    let calculator_window = app::App::default();
    // create app window
    let mut win = window::Window::default()
        .with_size(800, 600)
        .with_label("Function Calculator");
    win.make_resizable(true);

    // create the user Input fields
    let mut input_fields = Frame::new(0, 0, win.w(), win.h(), "");
    input_fields.set_frame(FrameType::BorderBox);
    input_fields.set_color(fltk::enums::Color::Light1);

    let mut positioning = Positioning {
        font_size: (win.h() + win.w()) / 100,

        v_rel_10: win.h() / 60,
        v_rel_20: win.h() / 30,
        v_rel_30: win.h() / 20,

        h_rel_10: win.h() / 80,
        h_rel_30: win.h() / 20,
        h_rel_80: win.h() / 10,

        input_parent_w: input_fields.w(),
        input_parent_h: input_fields.h(),

        function_input_w: win.w() - win.w() / 4,

        bool_inputs_y: 0,
        bool_inputs_w: 0,

        ck_btn_x: 0,
    };

    let function_a_input = function_a_input(&positioning);
    let function_b_input = function_b_input(&positioning, &function_a_input);
    let newton_interval_input = newton_interval_input(&positioning, &function_b_input);

    // positioning for bool inputs
    positioning.bool_inputs_y =
        newton_interval_input.y() + newton_interval_input.height() + positioning.v_rel_20;
    positioning.bool_inputs_w = win.w() / 3;
    positioning.ck_btn_x = (positioning.input_parent_w - positioning.bool_inputs_w) / 2;

    let root_ck_btn = root_ck_btn(&positioning);
    let extrema_ck_btn = extrema_ck_btn(&positioning, &root_ck_btn);
    let inflection_points_ck_btn = inflection_points_ck_btn(&positioning, &extrema_ck_btn);
    let y_value_ck_btn = y_value_ck_btn(&positioning, &inflection_points_ck_btn);
    let x_value_selector = x_value_selector(&positioning, &y_value_ck_btn);
    let intersection_points_ck_btn = intersection_pints_ck_btn(&positioning, &x_value_selector);
    let plot_function_ck_btn = plot_function_ck_btn(&positioning, &intersection_points_ck_btn);
    let calculate_btn = calculate_btn(&positioning, &plot_function_ck_btn);

    // pre-create the plot frame (hidden initially)
    let mut plot_frame = Frame::new(win.w(), 0, win.w(), win.h(), "");
    plot_frame.set_frame(fltk::enums::FrameType::BorderBox);
    plot_frame.set_color(fltk::enums::Color::White);
    plot_frame.hide();

    win.end();
    win.resizable(&plot_frame);
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
        plot_frame,
        calculator_window,
        win,
        input_fields,
    };

    widgets
}
