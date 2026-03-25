use crate::utils::simplify_function;
use crate::roots::get_root_of_function;

pub fn get_intersection_points(
    function_a: &Vec<f64>,
    function_b: &Vec<f64>,
    interval: &(f64, f64),
) -> Result<Vec<f64>, String> {
    // combine both functions into one by negating all coefficients of b
    let mut combined = function_a.clone();

    for i in (0..function_b.len()).step_by(2) {
        combined.push(-function_b[i]); // negate coefficient
        combined.push(function_b[i + 1]); // keep exponent
    }

    // simplify combines like terms (e.g. x^2 from both functions)
    let simplified = simplify_function(&combined);
    get_root_of_function(&simplified, interval)
}