use std::io;
use std::process;

pub fn run() {
    // Collect Data
    let x: [f64; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let y: [f64; 6] = [100.0, 200.0, 300.0, 400.0, 500.0, 600.0];
    // Compute Average of X
    let mut x_avg: f64 = 0.0;
    let mut x_total: f64= 0.0;
    let mut x_iter = 0;
    while x_iter != x.len() {
        x_total += x[x_iter];
        x_iter += 1;
    }
    x_avg /= x_total/(x.len() as f64);
    // Compute Average of Y
    // Calculate x^2, y^2, and x*y
    // Calculate b in y = mx + b
    // Calculate m in y = mx + b
}