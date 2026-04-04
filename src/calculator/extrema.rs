use crate::calculator::roots::get_root_of_function;
use crate::utils::{get_derivative_of_function, get_value_of_function};

pub fn calculate_extrema(
    function_variables: &Vec<f64>,
    newton_interval: &(f64, f64),
) -> Result<(Vec<f64>, Vec<f64>), String> {
    let derivative = get_derivative_of_function(&function_variables);
    let second_derivative = get_derivative_of_function(&derivative);
    let mut extrema: Vec<f64> = Vec::new();
    let mut saddle_points: Vec<f64> = Vec::new();

    let derivative_roots = get_root_of_function(&derivative, &newton_interval)?;

    for i in 0..derivative_roots.len() {
        let root = derivative_roots[i];
        if get_value_of_function(&second_derivative, &root).abs() > 1e-8 {
            extrema.push(root)
        } else {
            saddle_points.push(root);
        }
    }

    Ok((extrema, saddle_points))
}
