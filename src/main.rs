// This program is able to calculate the roots of any given polynomial
// The function must be supplied as f(x) = a_1*x^n_1 + a_2*x^n_2... as an array of variables with function_variables = {a_1, n_1, a_2, n_2...}
// The length of the array is variable

// My future plans are to make this a general calculator for every action with functions though right now I don't have the time

// =================================================================================================
// These are general functions used everywhere throughout the code

mod roots;
mod extrema;
mod inflection_points;

use std::f64::NAN;
use rand::{RngExt, SeedableRng};
use rand::rngs::StdRng;
use std::time::SystemTime;
use plotters::prelude::*;
use crate::roots::get_root_of_function;

// Generate the starting X-value
fn get_random_number(interval: &(f64, f64)) -> f64 {
    // Seeds the rng so that the number is different all the time
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    // generates the random number in an interval of -1000.0 to +1000.0
    let mut rng = StdRng::seed_from_u64(seed);
rng.random_range(interval.0..=interval.1)
}

// Calculates the Y-value of a function at a given X-value
fn get_value_of_function(function_variables: &Vec<f64>, x: &f64) -> f64 {
    let mut value = 0.0;

    for i in (0..function_variables.len()).step_by(2) {
        value += function_variables[i] * x.powf(function_variables[i + 1]);
    }

    value
}

// Calculates the derivative of a given function
fn get_derivative_of_function(function_variables: &Vec<f64>) -> Vec<f64> {
    let mut derivative_variables_raw = vec![];


    for i in (0..function_variables.len()).step_by(2) {
        let a = function_variables[i];
        let n = function_variables[i + 1];
        derivative_variables_raw.push(a * n);       // new a becomes old a * old n
        derivative_variables_raw.push(n - 1.0);     // new n becomes old n - 1.0
    }

    let derivative_variables = simplify_function(&derivative_variables_raw);
    derivative_variables
}

fn print(variables: &Vec<Vec<f64>>, calculate_root: bool, calculate_extrema: bool, calculate_inflection_points: bool) {
    /*
    user_variables: &Vec<f64>,
    function_variables: &Vec<f64>,
    roots: &Vec<f64>,
    extrema: Vec<f64>,
    saddle_points: Vec<f64>,
    inflection_points: Vec<f64>,
    calculate_root: bool,
    calculate_extrema: bool,
    calculate_inflection_points: bool
    */

    let user_variables = variables[0].clone();
    let function_variables = variables[1].clone();
    let roots = variables[2].clone();
    let extrema = variables[3].clone();
    let saddle_points = variables[4].clone();
    let inflection_points = variables[5].clone();

    // convert the variables of the function to a readable function
    let mut function_string = String::new();

    for i in (0..function_variables.len()).step_by(2) {
        let a = function_variables[i];
        let n = function_variables[i + 1];
        function_string += &format!("{}*x^{} + ", a, n);
    }


    let mut derivative_string = String::new();
    let derivative = get_derivative_of_function(&function_variables);

    for i in (0..derivative.len()).step_by(2) {
        let a = derivative[i];
        let n = derivative[i + 1];
        derivative_string += &format!("{}*x^{} + ", a, n);
    }


    let mut user_variables_string = String::new();

    for i in (0..user_variables.len()).step_by(2) {
        let a = user_variables[i];
        let n = user_variables[i + 1];
        user_variables_string += &format!("{}*x^{} + ", a, n);
    }

    // Remove the last " + "
    function_string.truncate(function_string.len() - 3);

    // remove unnecessary roots
    let mut root_string = Vec::new();

    for i in 0..roots.len() {
        if !roots[i].is_nan() {root_string.push(&roots[i]);}
    }

    println!();
    println!("------------------------------------------------------------");
    println!();
    println!("The user Input is:");
    println!("{}", user_variables_string);
    println!();
    println!("The function is:");
    println!("f(x) = {}", function_string);
    println!();
    println!("The derivative of the function is:");
    println!("f'(x) = {}", derivative_string);
    if calculate_root {
        println!();
        println!("The roots of the function are:");
        println!("{:#?}", root_string);
    }
    if calculate_extrema {
        println!();
        println!("The extrema of the function are:");
        println!("{:#?}", extrema);
        println!("The saddle points of the function are:");
        println!("{:#?}", saddle_points);
    }
    if calculate_inflection_points {
        println!();
        println!("The inflection points of the function are:");
        println!("{:#?}", inflection_points);
    }
    println!();
    println!("------------------------------------------------------------");
}

fn simplify_function(function_variables: &Vec<f64>) -> Vec<f64> {
    use std::collections::HashMap;

    let mut terms: HashMap<i64, f64> = HashMap::new();

    for i in (0..function_variables.len()).step_by(2) {
        let a = function_variables[i];
        let n = function_variables[i + 1];
        if n < 0.0 { continue; }
        let key = (n * 1000.0) as i64;
        *terms.entry(key).or_insert(0.0) += a;
    }

    // collect as pairs first, then sort by exponent descending
    let mut pairs: Vec<(f64, f64)> = terms
        .iter()
        .filter(|&(_, &a)| a != 0.0)
        .map(|(&key, &a)| (a, key as f64 / 1000.0))
        .collect();

    pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); // sort by exponent (index 1)

    // flatten back to [a, n, a, n...]
    pairs.iter().flat_map(|&(a, n)| vec![a, n]).collect()
}

// =================================================================================================
// This is the main function, the starting point of the entire program

fn main(){
    println!();
    println!("------------------------------------------------------------");
    println!();
    println!(
        "Welcome to the function calculator! This program is able to calculate the roots, extrema, saddle points and inflection points of any given polynomial.
        The function must be supplied as f(x) = a_1*x^n_1 + a_2*x^n_2... as an array of variables with function_variables = [a_1, n_1, a_2, n_2...].
        The length of the array is variable."
    );
    println!();
    println!("------------------------------------------------------------");
    println!();

    // user inputs: --------------------------------------------------------------------------------

    // Here the user may set the desired function
    let user_variables = vec![1.0, 4.0, -4.0, 1.0, 3.0, 1.0, 10.0, 1.0, -5.0, 0.0];

    // Here the user may set the desired interval
    let newton_interval: (f64, f64) = (-100.0, 100.0);

    // Here the user may choose what to calculate
    let calculate_root = true;
    let calculate_extrema = true;
    let calculate_inflection_points = true;

    // ---------------------------------------------------------------------------------------------

    // Check if the function is valid
    if user_variables.len() == 0 || user_variables.len() % 2 != 0 {
        println!("Invalid function");
        return;
    }

    let function_variables = simplify_function(&user_variables);

    // Plot the function using plotters (fixed for overflow)
    let plot_result = (|| -> Result<(), Box<dyn std::error::Error>> {
        // Use a smaller, reasonable x-range for plotting
        let x_min = -100.0;
        let x_max = 100.0;

        // Generate (x, y) points and filter out non-finite y values
        let points: Vec<(f64, f64)> = (0..=1000)
            .map(|i| {
                let x = x_min + (x_max - x_min) * (i as f64) / 1000.0;
                let y = get_value_of_function(&function_variables, &x);
                (x, y)
            })
            .filter(|(_, y)| y.is_finite())
            .collect();

        // Dynamically determine y_min and y_max from the points
        let (y_min, y_max) = points.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &(_, y)| {
            (min.min(y), max.max(y))
        });

        // If all y are non-finite, skip plotting
        if y_min == f64::INFINITY || y_max == f64::NEG_INFINITY {
            return Err("All y values are non-finite, cannot plot graph.".into());
        }

        let out = BitMapBackend::new("output.png", (640, 480)).into_drawing_area();
        out.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&out)
            .caption("Polynomial Graph", ("sans-serif", 30))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

        chart.configure_mesh().draw()?;

        chart.draw_series(LineSeries::new(points, &RED))?;
        Ok(())
    })();
    if let Err(e) = plot_result {
        println!("Plotting error: {}", e);
    }

    // Prepare print_variables with 5 slots, all set to vec![NAN] by default
    let mut print_variables: Vec<Vec<f64>> = vec![vec![NAN]; 6];
    print_variables[0] = user_variables.clone();
    print_variables[1] = function_variables.clone();

    // Roots
    if calculate_root {
        match get_root_of_function(&function_variables, &newton_interval) {
            Ok(root) => {
                print_variables[2] = root.clone();
            },
            Err(e) => {
                println!("{}", e);
                // Already NAN
            }
        }
    }

    // Extrema and saddle points
    if calculate_extrema {
        match extrema::calculate_extrema(&function_variables, &newton_interval) {
            Ok(extrema) => {
                print_variables[3] = extrema.0.clone();
                print_variables[4] = extrema.1.clone();
            },
            Err(e) => {
                println!("{}", e);
                // Already NAN
            }
        }
    }

    // Inflection points
    if calculate_inflection_points {
        match inflection_points::get_inflection_points(&function_variables, &newton_interval) {
            Ok(inflection_points) => {
                print_variables[5] = inflection_points.clone();
            },
            Err(e) => {
                println!("{}", e);
                // Already NAN
            }
        }
    }

    print(&print_variables, calculate_root.clone(), calculate_extrema.clone(), calculate_inflection_points.clone());
}
