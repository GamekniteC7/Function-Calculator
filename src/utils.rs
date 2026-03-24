use std::time::SystemTime;
use plotters::backend::BitMapBackend;
use plotters::chart::ChartBuilder;
use plotters::drawing::IntoDrawingArea;
use plotters::prelude::{LineSeries, RED, WHITE};
use rand::prelude::StdRng;
use rand::{RngExt, SeedableRng};

// Generate the starting X-value
pub(crate) fn get_random_number(interval: &(f64, f64)) -> f64 {
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
pub(crate) fn get_value_of_function(function_variables: &Vec<f64>, x: &f64) -> f64 {
    let mut value = 0.0;

    for i in (0..function_variables.len()).step_by(2) {
        value += function_variables[i] * x.powf(function_variables[i + 1]);
    }

    value
}



// Calculates the derivative of a given function
pub(crate) fn get_derivative_of_function(function_variables: &Vec<f64>) -> Vec<f64> {
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



pub(crate) fn simplify_function(function_variables: &Vec<f64>) -> Vec<f64> {
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



pub(crate) fn plot_function(function_variables: &Vec<f64>) -> Result<(), String> {
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
    match plot_result {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Plotting error: {}", e)),
    }
}



pub(crate) fn print(variables: &Vec<Vec<f64>>, calculate_root: bool, calculate_extrema: bool, calculate_inflection_points: bool, calculate_point: bool) {
    /*
    user_variables: &Vec<f64>,
    function_variables: &Vec<f64>,
    roots: &Vec<f64>,
    extrema: Vec<f64>,
    saddle_points: Vec<f64>,
    inflection_points: Vec<f64>,
    calculate_root: bool,
    calculate_extrema: bool,
    calculate_inflection_points: bool,
    calculate_point: bool
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
    derivative_string.truncate(derivative_string.len() - 3);
    user_variables_string.truncate(user_variables_string.len() - 3);

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
    if calculate_point {
        println!();
        println!("The y-value of the function at the x-value point is:");
        println!("{:#?}", variables[6]);
    }
    println!();
    println!("------------------------------------------------------------");
}