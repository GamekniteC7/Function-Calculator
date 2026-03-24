use crate::{get_derivative_of_function, roots::get_root_of_function};

pub fn get_inflection_points(function_variables: &Vec<f64>, newton_interval: &(f64, f64)) -> Result<Vec<f64>, String> {
    let second_derivative = Vec::from(get_derivative_of_function(&get_derivative_of_function(&function_variables)));
    let mut inflection_points: Vec<f64> = Vec::new();

    match get_root_of_function(&second_derivative, &newton_interval) {
        Ok(root) => { inflection_points = root; },
        Err(e) => return Err(e),
    }

    Ok(inflection_points)
}