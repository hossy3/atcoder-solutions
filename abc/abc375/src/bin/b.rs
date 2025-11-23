use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(f64, f64); n],
    }
    xy.insert(0, (0.0, 0.0));
    xy.push((0.0, 0.0));
    let result: f64 = xy
        .windows(2)
        .map(|v| ((v[0].0 - v[1].0).powi(2) + (v[0].1 - v[1].1).powi(2)).sqrt())
        .sum();
    println!("{result}");
}
