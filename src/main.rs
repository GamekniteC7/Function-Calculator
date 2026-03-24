mod roots;
mod extrema;
mod inflection_points;
mod utils;

use std::f64::NAN;

use crate::roots::get_root_of_function;
use crate::utils::{get_value_of_function, print, simplify_function, plot_function};


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
    let user_variables = vec![
        1.0, 10.0,
        -5.0,  9.0,
        -30.0,  8.0,
        150.0,  7.0,
        273.0,  6.0,
        -1365.0,  5.0,
        -820.0,  4.0,
        4100.0,  3.0,
        576.0,  2.0,
        -2880.0,  1.0,
    ];

    // Here the user may set the desired interval
    let newton_interval: (f64, f64) = (-100.0, 100.0);

    // Here the user may decide if the program should give the y-value of a function at a given y-value
    let calculate_point = true;
    let point = 10.0;

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

    match plot_function(&function_variables) {
        Ok(_) => {}
        Err(e) => {println!("{}", e);}
    }

    // Prepare print_variables with 5 slots, all set to vec![NAN] by default
    let mut print_variables: Vec<Vec<f64>> = vec![vec![NAN]; 7];
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

    // Value at a point
    if calculate_point {
        print_variables[6] = vec![get_value_of_function(&function_variables, &point)];
    }

    print(&print_variables, calculate_root, calculate_extrema, calculate_inflection_points, calculate_point);
}
