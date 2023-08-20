use proconio::input;

fn main() {
    input! {
        n: usize,
        pq: [(usize, usize); n],
    }
    let mut expected = 0.0;
    for &(p, q) in &pq {
        expected += (q as f64) / (p as f64);
    }
    println!("{}", expected);
}
