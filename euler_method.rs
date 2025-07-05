use std::fs::File;
use std::error::Error;
use std::io::Write;
use std::f64::consts::PI;

fn f(t: f64, y: f64) -> f64 {
    t.cos() - y
}

fn main() -> Result<(), Box<dyn Error>> {
    let t0 = 0.0;
    let y0 = 1.0;
    let t_end = 5.0;
    let n = 20;
    let h = (t_end - t0) / n as f64;

    let mut t_values = vec![t0];
    let mut y_values = vec![y0];

    let mut t = t0;
    let mut y = y0;

    for _ in 0..n {
        y += h * f(t, y);
        t += h;
        t_values.push(t);
        y_values.push(y);
    }

    // Write to CSV
    let mut file = File::create("euler_solution.csv")?;
    writeln!(file, "t,y")?;
    for (t, y) in t_values.iter().zip(y_values.iter()) {
        writeln!(file, "{:.6}, {:.6}", t, y)?;
    }

    println!("Results saved to 'euler_solution.csv'");
    Ok(())
}
