// =================================================================================================
// This part of the code uses the newton method to calculate the root of any given function
// The Code will only work in an interval which you will set manually in main in

use crate::utils::{get_derivative_of_function, get_random_number, get_value_of_function};

// calculates the tangent line for the point P( x | f(x) )
fn get_tangent(function_variables: &Vec<f64>, derivative_variables: &Vec<f64>, x: &mut f64, upper_interval: f64) -> Vec<f64> {
    let mut tangent_variables = Vec::new();

    // calculates the m of the function f(x) = mx+b
    tangent_variables.push(get_value_of_function(derivative_variables, &*x));
    // calculates the b of the function f(x) = mx+b
    tangent_variables.push(get_value_of_function(function_variables, &*x) - tangent_variables[0] * *x);


    while tangent_variables[0] == 0.0 && *x < upper_interval
        || is_singularity(&derivative_variables, *x) && *x < upper_interval
    {
        *x += 1.0;
        tangent_variables[0] = get_value_of_function(derivative_variables, &*x);
        tangent_variables[1] = get_value_of_function(function_variables, &*x) - tangent_variables[0] * *x;
    }

    tangent_variables
}

// Calculates the root of a linear function (tangent)
fn get_root(tangent_variables: &Vec<f64>) -> Result<f64, String> {
    if tangent_variables[0] == 0.0 {
        return Err(String::from("Slope is 0, cannot calculate root"));
    }

    Ok(-tangent_variables[1] / tangent_variables[0])
}

fn is_singularity(variables: &Vec<f64>, x: f64) -> bool {
    let d = get_value_of_function(variables, &x);
    d.is_nan() || d.is_infinite() || d.abs() > 1e10
}

fn newton_method(function_variables: &Vec<f64>, interval: &(f64, f64)) -> Result<f64, String> {
    let derivative_variables = get_derivative_of_function(&function_variables);
    let mut x = get_random_number(&interval);

    // Move x into valid domain if function is undefined (e.g. noninteger exponents with negative x)
    while get_value_of_function(&function_variables, &x).is_nan() && x < interval.1 {
        x += 1.0;
    }

    // If no valid x was found in the interval, report and exit
    if x >= interval.1 && get_value_of_function(&function_variables, &x).is_nan() {
        return Err("No valid x found in interval".to_string());
    }


    // Find the root
    for _i in 0..100 {
        let tangent_variables = get_tangent(&function_variables, &derivative_variables, &mut x, interval.1);
        match get_root(&tangent_variables) {
            Ok(root) => {
                if (root - x).abs() < 1e-10 {
                    x = root;
                    break;
                }
                x = root;
            },
            Err(e) => return Err(e),
        }
    }


    if get_value_of_function(&function_variables, &x).abs() > 1e-6 {
        return Err("Failed to find root within tolerance".to_string());
    }

    Ok(x) // ← now returns the actual root value
}

pub fn get_root_with_newton_method(
    function_variables: &Vec<f64>,
    interval: &(f64, f64),
    n_points: usize,
    tol: f64,
) -> Result<Vec<f64>, String> {
    if n_points < 2 {
        return Err("n_points must be at least 2".to_string());
    }

    let mut roots: Vec<f64> = Vec::new();

    let (x_start, x_end) = interval;
    let step = (x_end - x_start) / (n_points as f64 - 1.0);

    if get_value_of_function(function_variables, &0.0).abs() < 1e-10 {
        roots.push(0.0);
    }

    for i in 0..n_points {
        let x0 = x_start + i as f64 * step;
        let shifted_interval = (x0, interval.1);

        if let Ok(root) = newton_method(function_variables, &shifted_interval) {
            if !roots.iter().any(|r| (r - root).abs() < tol) {
                roots.push(root); // ← directly push the f64
            }
        }
    }

    if roots.is_empty() {
        return Err("No roots found in the given interval".to_string());
    }

    roots.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Ok(roots)
}

pub fn get_all_roots_with_bracketing(
    function_variables: &Vec<f64>,
    interval: &(f64, f64),
    n_points: usize,
    tol: f64,
) -> Result<Vec<f64>, String> {
    if n_points < 2 {
        return Err("n_points must be at least 2".to_string());
    }

    let mut roots: Vec<f64> = Vec::new();

    let (x_start, x_end) = interval;
    let step = (x_end - x_start) / (n_points as f64 - 1.0);

    let xs: Vec<f64> = (0..n_points).map(|i| x_start + i as f64 * step).collect();
    let ys: Vec<f64> = xs
        .iter()
        .map(|&x| get_value_of_function(function_variables, &x))
        .collect();

    if get_value_of_function(function_variables, &0.0).abs() < 1e-10 {
        roots.push(0.0);
    }

    for i in 0..n_points - 1 {
        if ys[i] * ys[i + 1] < 0.0 {
            let bracket = (xs[i], xs[i + 1]);

            if let Ok(root) = newton_method(function_variables, &bracket) {
                if !roots.iter().any(|r| (r - root).abs() < tol) {
                    roots.push(root); // ← directly push the f64
                }
            }
        }
    }

    if roots.is_empty() {
        return Err("No roots found in the given interval".to_string());
    }

    roots.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Ok(roots)
}

// =================================================================================================
// This part of the code uses the quartic formula to calculate the root of quartic functions

fn polynome_to_quartic(function_variables: &Vec<f64>) -> Vec<f64> {
    let mut a: f64 = 0.0; // x^4
    let mut b: f64 = 0.0; // x^3
    let mut c: f64 = 0.0; // x^2
    let mut d: f64 = 0.0; // x^1
    let mut e: f64 = 0.0; // x^0

    for chunk in function_variables.chunks(2) {
        if chunk[1] == 4.0      { a += chunk[0]; }
        else if chunk[1] == 3.0 { b += chunk[0]; }
        else if chunk[1] == 2.0 { c += chunk[0]; }
        else if chunk[1] == 1.0 { d += chunk[0]; }
        else if chunk[1] == 0.0 { e += chunk[0]; }
    }

    vec![a, b, c, d, e]
}

// ax^4 = 0 → x = 0
fn quartic_no_bcde(f: &Vec<f64>) -> Result<Vec<f64>, String> {
    if f[0] == 0.0 { return Err("No root".to_string()); }
    Ok(vec![0.0])
}

// ax^4 + e = 0 → x = ±(-e/a)^0.25
fn quartic_no_bcd(f: &Vec<f64>) -> Result<Vec<f64>, String> {
    if f[0] == 0.0 { return Err("No root".to_string()); }
    let val = -f[4] / f[0];
    if val < 0.0 { return Err("No real roots".to_string()); }
    let r = val.sqrt().sqrt();
    Ok(vec![r, -r])
}

// ax^4 + cx^2 = 0 → x^2(ax^2 + c) = 0 → x=0 or x=±sqrt(-c/a)
fn quartic_no_bde(f: &Vec<f64>) -> Result<Vec<f64>, String> {
    if f[0] == 0.0 { return Err("No root".to_string()); }
    let mut roots = vec![0.0];
    let val = -f[2] / f[0];
    if val >= 0.0 {
        let r = val.sqrt();
        roots.push(r);
        roots.push(-r);
    }
    Ok(roots)
}

// ax^4 + dx = 0 → x(ax^3 + d) = 0 → x=0 or x=(-d/a)^(1/3)
fn quartic_no_bce(f: &Vec<f64>) -> Result<Vec<f64>, String> {
    if f[0] == 0.0 { return Err("No root".to_string()); }
    let r = (-f[3] / f[0]).cbrt();
    Ok(vec![0.0, r])
}

// ax^4 + cx^2 + e = 0 → substitue u = x^2 → quadratic in u
fn quartic_no_bd(f: &Vec<f64>) -> Result<Vec<f64>, String> {
    if f[0] == 0.0 { return Err("No root".to_string()); }
    let disc = f[2] * f[2] - 4.0 * f[0] * f[4];
    if disc < 0.0 { return Err("No real roots".to_string()); }
    let u1 = (-f[2] + disc.sqrt()) / (2.0 * f[0]);
    let u2 = (-f[2] - disc.sqrt()) / (2.0 * f[0]);
    let mut roots = vec![];
    if u1 >= 0.0 { roots.push(u1.sqrt()); roots.push(-u1.sqrt()); }
    if u2 >= 0.0 { roots.push(u2.sqrt()); roots.push(-u2.sqrt()); }
    if roots.is_empty() { return Err("No real roots".to_string()); }
    Ok(roots)
}

pub fn get_root_quartic(function_variables: &Vec<f64>, newton_interval: &(f64, f64)) -> Result<Vec<f64>, String> {
    let f = polynome_to_quartic(function_variables);
    // [a, b, c, d, e] → f[0]=a, f[1]=b, f[2]=c, f[3]=d, f[4]=e

    let roots = if f[1] == 0.0 && f[2] == 0.0 && f[3] == 0.0 && f[4] == 0.0 {
        quartic_no_bcde(&f)?
    } else if f[1] == 0.0 && f[2] == 0.0 && f[3] == 0.0 {
        quartic_no_bcd(&f)?
    } else if f[1] == 0.0 && f[3] == 0.0 && f[4] == 0.0 {
        quartic_no_bde(&f)?
    } else if f[1] == 0.0 && f[2] == 0.0 && f[4] == 0.0 {
        quartic_no_bce(&f)?
    } else if f[1] == 0.0 && f[3] == 0.0 {
        quartic_no_bd(&f)?
    } else {
        // general case — use the quartic formula, fall back to Newton
        let a = f[0]; let b = f[1]; let c = f[2]; let d = f[3]; let e = f[4];
        let p = (8.0 * a * c - 3.0 * b * b) / (8.0 * a * a);
        let q = (b * b * b - 4.0 * a * b * c + 8.0 * a * a * d) / (8.0 * a * a * a);
        let delta_0 = c * c - 3.0 * b * d + 12.0 * a * e;
        let delta_1 = 2.0 * c * c * c - 9.0 * b * c * d + 27.0 * b * b * e + 27.0 * a * d * d - 72.0 * a * c * e;
        let disc = delta_1 * delta_1 - 4.0 * delta_0 * delta_0 * delta_0;

        if disc < 0.0 || big_q_zero(&delta_0, &delta_1) {
            // quartic formula unstable, fall back to Newton
            get_root_with_newton_method(function_variables, newton_interval, 50, 1e-8)?
        } else {
            let big_q = ((delta_1 + disc.sqrt()) / 2.0).cbrt();
            let s = 0.5 * ((-2.0 / 3.0 * p + (big_q + delta_0 / big_q) / (3.0 * a)).sqrt());
            let r1 = -b/(4.0*a) - s + 0.5*((-4.0*s*s - 2.0*p + q/s).sqrt());
            let r2 = -b/(4.0*a) - s - 0.5*((-4.0*s*s - 2.0*p + q/s).sqrt());
            let r3 = -b/(4.0*a) + s + 0.5*((-4.0*s*s - 2.0*p - q/s).sqrt());
            let r4 = -b/(4.0*a) + s - 0.5*((-4.0*s*s - 2.0*p - q/s).sqrt());
            vec![r1, r2, r3, r4]
        }
    };

    Ok(roots)
}

fn big_q_zero(delta_0: &f64, delta_1: &f64) -> bool {
    let big_q = ((delta_1 + (delta_1 * delta_1 - 4.0 * delta_0 * delta_0 * delta_0).sqrt()) / 2.0).cbrt();
    big_q.abs() < 1e-10
}

// =================================================================================================
// This part of the code uses Cardano's formula to calculate the root of cubic functions

fn polynome_to_cubic(function_variables: &Vec<f64>) -> Vec<f64> {
    let mut a:f64 = 0.0;
    let mut b:f64 = 0.0;
    let mut c:f64 = 0.0;
    let mut d:f64 = 0.0;

    for chunk in function_variables.chunks(2) {
        if chunk[1] == 3.0 { a += chunk[0]; }
        else if chunk[1] == 2.0 { b += chunk[0]; }
        else if chunk[1] == 1.0 { c += chunk[0]; }
        else if chunk[1] == 0.0 { d += chunk[0]; }
    }

    vec![a, b, c, d]
}

fn cardanos_formula(function_variables: &Vec<f64>) -> Result<Vec<f64>, String> {
    let mut root:Vec<f64> = Vec::new();
    let a = function_variables[0];
    let b = function_variables[1];
    let c = function_variables[2];
    let d = function_variables[3];

    let p = ((3.0*a*c-b.powf(2.0))) / (3.0*a.powf(2.0));
    let q = ((2.0*b.powf(3.0)-9.0*a*b*c+27.0*a.powf(2.0)*d)) / (27.0*a.powf(3.0));

    if (p/3.0).powf(3.0) + (q/2.0).powf(2.0) < 0.0 {
        return Err(String::from("not possible with Cardano's method. Fall back to Newton's method"));
    }
    else {
        let t = (-q / 2.0 + ((p / 3.0).powf(3.0) + (q / 2.0).powf(2.0)).sqrt()).cbrt() + (-q / 2.0 - ((p / 3.0).powf(3.0) + (q / 2.0).powf(2.0)).sqrt()).cbrt();
        root.push(t - b / (3.0 * a));
    }

    Ok(root)
}

fn cubic_no_bc(function_variables: &Vec<f64>) -> Result<Vec<f64>, String> {
    let mut root:Vec<f64> = vec![0.0];

    if function_variables[0] == 0.0 {
        return Err(String::from("function has no root"));
    }
    else {
        root[0] = (-function_variables[3] / function_variables[0]).cbrt();
    }
    Ok(root)
}

fn cubic_no_bd(function_variables: &Vec<f64>) -> Result<Vec<f64>, String> {
    let mut root:Vec<f64> = vec![0.0, 0.0, 0.0];

    if function_variables[0] == 0.0 || -function_variables[2] / function_variables[0] <= 0.0 {
        return Err(String::from("function has no root"));
    }
    else {
        root[0] = (-function_variables[2] / function_variables[0]).sqrt();
        root[1] = -(-function_variables[2] / function_variables[0]).sqrt();
    }
    Ok(root)
}

fn cubic_no_cd(function_variables: &Vec<f64>) -> Result<Vec<f64>, String> {
    let mut root:Vec<f64> = vec![0.0, 0.0];

    if function_variables[0] == 0.0 {
        return Err(String::from("function has no root"));
    }
    else {
        root[1] = -function_variables[1] / function_variables[0];
    }
    Ok(root)
}

fn cubic_no_d(function_variables: &Vec<f64>) -> Result<Vec<f64>, String> {
    let mut root:Vec<f64> = vec![0.0, 0.0, 0.0];
    let quadratic_variables:Vec<f64> = vec![function_variables[0], 2.0, function_variables[1], function_variables[2]];

    if function_variables[0] == 0.0 {
        return Err(String::from("function has no root"));
    }
    match quadratic_formula(&quadratic_variables) {
        Ok(roots) => {root[0] = roots[0]; root[1] = roots[1];
        },
        Err(e) => println!("Error: {}", e)
    }

    Ok(root)
}

fn cubic_no_bcd(function_variables: &Vec<f64>) -> Result<Vec<f64>, String> {
    let root:Vec<f64> = vec![0.0];

    if function_variables[0] == 0.0 {
        return Err(String::from("function has no root"));
    }
    Ok(root)
}

pub fn get_root_cubic(function_variables: &Vec<f64>, newton_interval: &(f64, f64)) -> Result<Vec<f64>, String> {
    let cubic_function: Vec<f64> = polynome_to_cubic(&function_variables);
    let roots = if cubic_function[1] == 0.0 && cubic_function[2] == 0.0 && cubic_function[3] == 0.0 {
        cubic_no_bcd(&cubic_function)?
    } else if cubic_function[1] == 0.0 && cubic_function[2] == 0.0 {
        cubic_no_bc(&cubic_function)?
    } else if cubic_function[1] == 0.0 && cubic_function[3] == 0.0 {
        cubic_no_bd(&cubic_function)?
    } else if cubic_function[2] == 0.0 && cubic_function[3] == 0.0 {
        cubic_no_cd(&cubic_function)?
    } else if cubic_function[3] == 0.0 {
        cubic_no_d(&cubic_function)?
    } else {
        match cardanos_formula(&cubic_function) {
            Ok(roots) => roots,
            Err(_) => get_root_with_newton_method(function_variables, &newton_interval, 50, 1e-8)?, // fallback
        }
    };
    Ok(roots)
}

// =================================================================================================
// This part of the code uses the quadratic formula to calculate the root of a quadratic function

fn polynome_to_quadratic(function_variables: &Vec<f64>) -> Vec<f64> {
    let mut a:f64 = 0.0;
    let mut b:f64 = 0.0;
    let mut c:f64 = 0.0;

    for chunk in function_variables.chunks(2) {
        if chunk[1] == 2.0      { a += chunk[0]; }
        else if chunk[1] == 1.0 { b += chunk[0]; }
        else if chunk[1] == 0.0 { c += chunk[0]; }
    }

    vec![a, 2.0, b, c]
}

fn quadratic_formula(quadratic_variables: &Vec<f64>) -> Result<Vec<f64>, String> {
    let mut roots:Vec<f64> = vec![0.0, 0.0];
    let mut quadratic:Vec<f64> = quadratic_variables.clone();

    quadratic[2] /= quadratic[0];
    quadratic[3] /= quadratic[0];

    if (quadratic[2]/2.0).powf(2.0) - quadratic[3] <= 0.0 {
        return Err(String::from("function has no root"))
    }
    else {
        roots[0] = -(quadratic[2] / 2.0) + ((quadratic[2] / 2.0).powf(2.0) - quadratic[3]).sqrt();
        roots[1] = -(quadratic[2] / 2.0) - ((quadratic[2] / 2.0).powf(2.0) - quadratic[3]).sqrt();
    }

    Ok(roots)
}

fn quadratic_only_c(quadratic_variables: &Vec<f64>) -> Result<Vec<f64>, String> {
    let mut roots:Vec<f64> = vec![0.0, 0.0];

    if quadratic_variables[0] == 0.0 || -quadratic_variables[3] / quadratic_variables[0] <= 0.0 {
        return Err(String::from("function has no root"));
    }
    else {
        roots[0] = (-quadratic_variables[3] / quadratic_variables[0]).sqrt();
        roots[1] = -(-quadratic_variables[3] / quadratic_variables[0]).sqrt();
    }

    Ok(roots)
}

fn quadratic_only_bx(quadratic_variables: &Vec<f64>) -> Result<Vec<f64>, String> {
    let mut roots:Vec<f64> = vec![0.0, 0.0];

    if quadratic_variables[0] == 0.0 {
        return Err(String::from("function has no root"));
    }
    else {
        roots[1] = -quadratic_variables[2] / quadratic_variables[0]
    }

    Ok(roots)
}

fn quadratic_only_ax(quadratic_variables: &Vec<f64>) -> Result<Vec<f64>, String> {
    let mut roots:Vec<f64> = vec![0.0];

    if quadratic_variables[0] == 0.0 {
        return Err(String::from("function has no root"));
    }
    else {
        roots[0] = 0.0;
    }

    Ok(roots)
}

pub fn get_root_quadratic(function_variables: &Vec<f64>) -> Result<Vec<f64>, String> {
    let quadratic_function:Vec<f64> = polynome_to_quadratic(&function_variables);

    if quadratic_function[2] == 0.0 && quadratic_function[3] == 0.0 {
        quadratic_only_ax(&quadratic_function)
    }
    else if quadratic_function[2] == 0.0 {
        quadratic_only_c(&quadratic_function)
    }
    else if quadratic_function[3] == 0.0 {
        quadratic_only_bx(&quadratic_function)
    }
    else {
        quadratic_formula(&quadratic_function)
    }
}

// =================================================================================================
// This part of the code calculates the root of linear equations

pub fn get_root_linear(function_variables: &Vec<f64>) -> Result<Vec<f64>, String> {
    // If function only has two variables (a, n) then function is either constant or nonlinear
    // If function is not constant return root
    let m:f64 = function_variables.chunks(2).filter(|t| t[1] == 1.0).map(|t| t[0]).sum();
    let b:f64 = function_variables.chunks(2).filter(|t| t[1] == 0.0).map(|t| t[0]).sum();

    if m == 0.0 {
        Err(String::from("The function is constant"))
    } else {
        let root = -b / m;
        Ok(vec![root])
    }
}

// =================================================================================================
// This part of the code Is the function deciding how to calculate the root

pub fn get_root_of_function(function_variables: &Vec<f64>, newton_interval: &(f64, f64)) -> Result<Vec<f64>, String> {

    // check if the function is linear
    let is_linear = function_variables.chunks(2).all(|t| t[1] == 0.0 || t[1] == 1.0);

    // check if the function is quadratic
    let is_quadratic = function_variables.chunks(2).all(|t| t[1] == 0.0 || t[1] == 1.0 || t[1] == 2.0)
        && function_variables.chunks(2).any(|t| t[1] == 2.0);

    // check if the function is cubic
    let is_cubic = function_variables.chunks(2).all(|t| t[1] == 0.0 || t[1] == 1.0 || t[1] == 2.0 || t[1] == 3.0)
        && function_variables.chunks(2).any(|t| t[1] == 3.0);

    // check if the function is quartic
    let is_quartic = function_variables.chunks(2).all(|t| t[1] == 0.0 || t[1] == 1.0 || t[1] == 2.0 || t[1] == 3.0 || t[1] == 4.0)
        && function_variables.chunks(2).any(|t| t[1] == 4.0);

    // read the checks and act accordingly
    // if the function is linear there is no need to use the newton method as there is a general formula to calculate the root of linear functions
    if is_linear {
        get_root_linear(&function_variables)
    }
    // if the function is quadratic there is no need to use the newton method as there is a general formula to calculate the root of quadratic functions
    else if is_quadratic {
        get_root_quadratic(&function_variables)
    }
    // if the function is cubic there is no need to use the newton method as there is a general formula to calculate the root of cubic functions
    else if is_cubic {
        get_root_cubic(&function_variables, &newton_interval)
    }
    // if the function is quartic there is no need to use the newton method as there is a general formula to calculate the root of quartic functions
    else if is_quartic {
        get_root_quartic(&function_variables, &newton_interval)
    }

    // if there is no easier way to calculate the root of the function use the newton method
    else {
        get_all_roots_with_bracketing(&function_variables, &newton_interval, 5000, 1e-8)
    }
}
