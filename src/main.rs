// This program is able to calculate the roots of any given polynomial
// The function must be supplied as f(x) = a_1*x^n_1 + a_2*x^n_2... as an array of variables with function_variables = {a_1, n_1, a_2, n_2...}
// The length of the array is variable

// My future plans are to make this a general calculator for every action with functions though right now I don't have the time

// =================================================================================================
// These are general functions used everywhere throughout the code

mod root;
mod extrema;

use std::f64::NAN;
use rand::{RngExt, SeedableRng};
use rand::rngs::StdRng;
use std::time::SystemTime;
use crate::root::get_root_of_function;

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
fn get_value_of_function(function_variables: &Vec<f64>, x: f64) -> f64 {
    let mut value = 0.0;

    for i in (0..function_variables.len()).step_by(2) {
        value += function_variables[i] * x.powf(function_variables[i + 1]);
    }

    value
}

// Calculates the derivative of a given function
fn get_derivative_of_function(function_variables: &Vec<f64>) -> Vec<f64> {
    let mut derivative_variables = vec![];

    for i in (0..function_variables.len()).step_by(2) {
        let a = function_variables[i];
        let n = function_variables[i + 1];
        derivative_variables.push(a * n);       // new a becomes old a * old n
        derivative_variables.push(n - 1.0);     // new n becomes old n - 1.0
    }

    derivative_variables
}

fn print(variables: &Vec<Vec<f64>>, calculate_root: bool, calculate_extrema: bool) {
    /*
    function_variables: &Vec<f64>,
    roots: &Vec<f64>,
    extrema: Vec<f64>,
    saddle_points: Vec<f64>,
    calculate_root: bool,
    calculate_extrema: bool
    */

    let function_variables = variables[0].clone();
    let roots = variables[1].clone();
    let extrema = variables[2].clone();
    let saddle_points = variables[3].clone();

    // convert the variables of the function to a readable function
    let mut function_string = String::new();

    for i in (0..function_variables.len()).step_by(2) {
        let a = function_variables[i];
        let n = function_variables[i + 1];
        function_string += &format!("{}*x^{} + ", a, n);
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
    println!("The function was:");
    println!("f(x) = {}", function_string);
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
    println!();
    println!("------------------------------------------------------------");
}

// =================================================================================================
// This is the main function, the starting point of the entire program

fn main(){
    println!();
    println!("------------------------------------------------------------");
    println!();
    println!(
        "Welcome to the function calculator! This program is able to calculate the roots and extrema of any given polynomial.
        The function must be supplied as f(x) = a_1*x^n_1 + a_2*x^n_2... as an array of variables with function_variables = [a_1, n_1, a_2, n_2...].
        The length of the array is variable."
    );
    println!();
    println!("------------------------------------------------------------");
    println!();

    // user inputs: --------------------------------------------------------------------------------

    // Here the user may set the desired function
    let function_variables = vec![1.0, 3.0, -4.0, 3.0, 3.0, 2.0, 10.0, 1.0, -5.0, 0.0];

    // Here the user may set the desired interval
    let newton_interval: (f64, f64) = (-1000.0, 1000.0);

    // Here the user may choose what to calculate
    let calculate_root = true;
    let calculate_extrema = true;

    // ---------------------------------------------------------------------------------------------

    // Check if the function is valid
    if function_variables.len() == 0 || function_variables.len() % 2 != 0 {
        println!("Invalid function");
        return;
    }

    let mut print_variables:Vec<Vec<f64>> = vec![];
    print_variables.push(function_variables.clone());

    if calculate_root {
        match get_root_of_function(&function_variables, newton_interval) {
            Ok(root) => {
                print_variables.push(root.clone());
            },
            Err(e) => {
                println!("{}", e);
                print_variables.push(vec![NAN]);
            }
        }
    }

    if calculate_extrema {
        match extrema::calculate_extrema(&function_variables) {
            Ok(extrema) => {
                print_variables.push(extrema.0.clone());
                print_variables.push(extrema.1.clone());
            },
            Err(e) => {
                println!("{}", e);
                print_variables.push(vec![NAN]);
            }
        }
    }

    print(&print_variables, calculate_root.clone(), calculate_extrema.clone());
}
