use std::f64::consts::PI;

use proconio::input;

fn pitch(x: f64, y: f64, uy: f64, uz: f64) -> f64 {
    let x0 = (x.powi(2) + (uy - y).powi(2)).sqrt();
    uz.atan2(x0).to_degrees()
}

// returns (y, z)
fn pos(t: f64, l: f64, e: f64) -> (f64, f64) {
    let rad = e / t * PI * 2.;
    let y = -rad.sin() * l * 0.5;
    let z = (1. - rad.cos()) * l * 0.5;
    (y, z)
}

fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        e: [f64; q],
    }
    for e in e {
        let (uy, uz) = pos(t, l, e);
        let result = pitch(x, y, uy, uz);
        println!("{result}");
    }
}
