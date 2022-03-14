/*-------------------------------------------------
    Time Integration:
Ref.
[1] https://programing.style/archives/669
[2] https://kiito.hatenablog.com/entry/2019/02/11/172125
-------------------------------------------------*/
use gnuplot::{Caption, Color, Figure};
use std::{error, fs::File};
use std::io::prelude::*;
use std::time::Instant;
use std::io::BufWriter;

pub struct System {
    k: f64,       // spring [mN/mm]
    c: f64,       // damper [N/(mm/ms)]
    m: f64,       // mass [kg]
    x0: f64,      // initial position [mm]
    v0: f64,      // v0 = dx/dt|t=0
    n: i32,       // max loop number
    delta_t: f64, // dt [s]
}

impl System {
    // Define dy/dx = f(x)
    fn f(&self, x: f64, v: f64, _t: f64) -> f64 {
        (-self.c * v - self.k * x) / self.m
    }
}

impl System {
    // Runge-Kutta 4th order
    fn runge_kutta(&self, x: f64, v: f64, t: f64) -> (f64, f64) {
        let k1v = self.f(x, v, t) * self.delta_t;
        let k1x = v * self.delta_t;
        let k2x = (v + k1v / 2.0) * self.delta_t;
        let k2v = self.f(x + k1x / 2.0, v + k1v / 2.0, t + self.delta_t / 2.0) * self.delta_t;
        let k3v = self.f(x + k2x / 2.0, v + k2v / 2.0, t + self.delta_t / 2.0) * self.delta_t;
        let k3x = (v + k2v / 2.0) * self.delta_t;
        let k4v = self.f(x + k3x, v + k3v, t + self.delta_t) * self.delta_t;
        let k4x = (v + k3v) * self.delta_t;
        let dv = (k1v + 2.0 * k2v + 2.0 * k3v + k4v) / 6.0;
        let dx = (k1x + 2.0 * k2x + 2.0 * k3x + k4x) / 6.0;
        (dx, dv)
    }

    // fn leap_frog(&self, x: f64, v: f64, t: f64) -> (f64, f64) {
    //     let k1v = self.f(x, v, t) * self.delta_t;
    //     let k1x = v * self.delta_t;
    //     let k2x = (v + k1v / 2.0) * self.delta_t;
    //     let k2v = self.f(x + k1x / 2.0, v + k1v / 2.0, t + self.delta_t / 2.0) * self.delta_t;
    //     let dv = (k1v + 2.0 * k2v + 2.0 * k2v ) / 6.0;
    //     let dx = (k1x + 2.0 * k2x + 2.0 * k2x ) / 6.0;
    //     (dx, dv)
    // }
}

pub fn main() {
    let start = Instant::now();
    // Initialize
    let params = System {
        k: 1.0,        // [mN/mm]
        c: 1.0,        // [N/(mm/ms)]
        m: 1.0,        // [kg]
        x0: 0.0,       // [mm]
        v0: 1.0,       // [mm/ms]
        n: 1000,       // loop
        delta_t: 0.01, // [ms]
    };

    // file path
    let file_path: &str = "dist/output";
    let img_path: &str = "img/output.png";

    // Initialize
    let (mut x, mut v) = (params.x0, params.v0);

    let mut ts = vec![];
    let mut vs = vec![];
    let mut xs = vec![];

    // Iteration
    for i in 0..=params.n {
        let t = (i as f64) * params.delta_t;
        let (dx, dv) =
            params.runge_kutta(x, v, t);
            // params.leap_frog(i, x, v, t);

        v += dv;
        x += dx;

        ts.push(t);
        vs.push(v);
        xs.push(x);
    }
    let tp = ts.clone();
    let xp = xs.clone();

    // csv
    let n = params.n as usize;
    write_csv(n, ts, vs, xs, file_path)
            .map_err(|e| println!("{}", e))
            .ok();

    // image
    plot(tp, xp, img_path);
    // CPU time
    time(start);
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

fn write_csv(n: usize, t: Vec<f64>, v: Vec<f64>, x: Vec<f64>, file_path: &str) 
    -> Result<(), Box<dyn error::Error>> {
    let file_name: String = format!("{}.csv", file_path);
    let file = File::create(file_name).unwrap();

    // write text in the file
    let mut w = BufWriter::new(file);

    // header
    write!(w, "t, v, x\n").unwrap();
    // write csv
    for i in 0..n {
        let s = format!(
            "{},{},{}\n",
            t[i], v[i], x[i],
        );
        write!(w, "{}", s).unwrap();
    }

    w.flush().unwrap();
    Ok(())
}

fn plot(x: Vec<f64>, y: Vec<f64>, img_path: &str) {
    let mut fg = Figure::new();
    let img_path = img_path;

    fg.set_terminal("pngcairo", img_path);
    fg.axes2d().lines(&x, &y, &[Caption("EOM"), Color("blue")]);
    fg.show().unwrap();
}
