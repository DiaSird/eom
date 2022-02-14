/*-------------------------------------------------
    Time Integration:
Ref.
[1] https://programing.style/archives/669
[2] https://kiito.hatenablog.com/entry/2019/02/11/172125
-------------------------------------------------*/
use gnuplot::{Caption, Color, Figure};
use std::fs;
use std::io::prelude::*;
use std::time::Instant;

// parameters
const K: f64 = 1.0; // spring [mN/mm]
const C: f64 = 1.0; // damper [N/(mm/ms)]
const M: f64 = 1.0; // mass [kg]

// Initial setup
const X0: f64 = 0.0;
const V0: f64 = 1.0; // V0 = dx/dt|t=0

// step size
const N: i32 = 1000; // max loop number
const DELTA_T: f64 = 0.01; // dt [s]

// file path
const FILE_PATH: &str = "dist/output.csv";
const IMG_PATH: &str = "img/output.png";

pub fn main() {
    let start = Instant::now();
    // Initialize
    let (mut x, mut v) = (X0, V0);
    let mut xs = vec![];
    let mut ts = vec![];
    // Iteration
    for i in 1..N {
        let t = (i as f64) * DELTA_T;
        // Runge-Kutta 4th
        let (dx, dv) = runge_kutta(x, v, t);
        x += dx;
        v += dv;
        // leap_frog(i, x, v, t);
        // x += x;
        // v += v;
        // println!("{:<5.3}", x);
        ts.push(t);
        xs.push(x);
    }
    // output last results
    output(x, v, (N as f64) * DELTA_T, FILE_PATH);
    // image
    plot(ts, xs, IMG_PATH);
    // CPU time
    time(start);
}

// Define dy/dx = f(x)
fn f(x: f64, v: f64, _t: f64) -> f64 {
    -(C * v) / M - (K * x) / M
}

// measure CPU time
fn time(start: Instant) {
    let end = start.elapsed();
    println!(
        "All completed in {}.{:03}s.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}

fn runge_kutta(x: f64, v: f64, t: f64) -> (f64, f64) {
    let k1v = f(x, v, t) * DELTA_T;
    let k1x = v * DELTA_T;
    let k2x = (v + k1v / 2.0) * DELTA_T;
    let k2v = f(x + k1x / 2.0, v + k1v / 2.0, t + DELTA_T / 2.0) * DELTA_T;
    let k3v = f(x + k2x / 2.0, v + k2v / 2.0, t + DELTA_T / 2.0) * DELTA_T;
    let k3x = (v + k2v / 2.0) * DELTA_T;
    let k4v = f(x + k3x, v + k3v, t + DELTA_T) * DELTA_T;
    let k4x = (v + k3v) * DELTA_T;
    let dv = (k1v + 2.0 * k2v + 2.0 * k3v + k4v) / 6.0;
    let dx = (k1x + 2.0 * k2x + 2.0 * k3x + k4x) / 6.0;
    (dx, dv)
}

fn leap_frog(i: i32, mut x: f64, mut v: f64, t: f64) -> (f64, f64) {
    // Euler
    if i == 0 {
        x = x + f(x, v, t) * DELTA_T;
    }
    // leap-frog
    v = v + 2.0 * f(x, v, t + DELTA_T) * DELTA_T;
    x = x + 2.0 * v * DELTA_T;
    (x, v)
}

fn output(x: f64, v: f64, t: f64, file_path: &str) {
    // create a file
    let mut file = fs::File::create(file_path).unwrap();
    // write text in the file
    file.write_all(b"t, v, x\n");
    file.write_all(format!("{}, {}, {},", t, v, x).as_bytes())
        .unwrap();
}

fn plot(x: Vec<f64>, y: Vec<f64>, img_path: &str) {
    let mut fg = Figure::new();
    let img_path = img_path;
    fg.set_terminal("pngcairo", img_path);
    fg.axes2d().lines(&x, &y, &[Caption("EOM"), Color("blue")]);
    fg.show();
}
