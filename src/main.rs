mod roots;
mod extrema;
mod inflection_points;
mod utils;
mod intersection_points;

use std::f64::NAN;
use crate::intersection_points::get_intersection_points;
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
    let user_function_a = vec![
        2.0, 3.0,
        -5.0, 2.0,
        1.0, 1.0,
        6.0, 0.0
    ];

    let user_function_b = vec![
        3.0, 0.0
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
    let calculate_intersection_points = true;

    // ---------------------------------------------------------------------------------------------

    // Check if function a is valid
    if user_function_a.len() == 0 || user_function_a.len() % 2 != 0 {
        println!("Invalid function a");
        return;
    }

    // Check if function b is valid
    if user_function_b.len() == 0 || user_function_b.len() % 2 != 0 {
        println!("Invalid function b");
        return;
    }

    let function_variables_a = simplify_function(&user_function_a);
    let function_variables_b = simplify_function(&user_function_b);

    match plot_function(&function_variables_a) {
        Ok(_) => {}
        Err(e) => {println!("{}", e);}
    }

    // Prepare print_variables with 5 slots, all set to vec![NAN] by default
    let mut print_variables: Vec<Vec<f64>> = vec![vec![NAN]; 8];
    print_variables[0] = user_function_a.clone();
    print_variables[1] = function_variables_a.clone();

    // Roots
    if calculate_root {
        match get_root_of_function(&function_variables_a, &newton_interval) {
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
        match extrema::calculate_extrema(&function_variables_a, &newton_interval) {
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
        match inflection_points::get_inflection_points(&function_variables_a, &newton_interval) {
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
        print_variables[6] = vec![get_value_of_function(&function_variables_a, &point)];
    }

    // Calculate intersection
    if calculate_intersection_points {
        match get_intersection_points(&function_variables_a, &function_variables_b, &newton_interval) {
            Ok(intersection_points) => {
                print_variables[7] = intersection_points.clone();
            },
            Err(e) => {
                println!("{}", e);
                // Already NAN
            }
        }
    }

    print(&print_variables, calculate_root, calculate_extrema, calculate_inflection_points, calculate_point, calculate_intersection_points);
}
