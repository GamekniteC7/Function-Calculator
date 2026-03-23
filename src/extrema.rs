use crate::{get_value_of_function, get_derivative_of_function, root::get_root_of_function};

pub fn calculate_extrema(function_variables: &Vec<f64>,) -> Result<(Vec<f64>, Vec<f64>), String> {
    let derivative = Vec::from(get_derivative_of_function(&function_variables));
    let mut derivative_roots = Vec::new();
    let mut extrema:Vec<f64> = Vec::new();
    let mut saddle_points:Vec<f64> = Vec::new();

    match get_root_of_function(&derivative, (-1000.0, 1000.0)) {
        Ok(root) => {derivative_roots = root;},
        Err(e) => return Err(e),
    }

    for i in 0..derivative_roots.len() {
        if get_value_of_function(function_variables, derivative_roots[i]) != 0.0 {extrema.push(derivative_roots[i])}
        else {saddle_points.push(derivative_roots[i]);}
    }

    Ok((extrema, saddle_points))
}