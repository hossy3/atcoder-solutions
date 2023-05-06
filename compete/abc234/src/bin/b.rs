use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut result2 = 0;
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let len2 = (xy[j].0 - xy[i].0).pow(2) + (xy[j].1 - xy[i].1).pow(2);
            result2 = result2.max(len2);
        }
    }
    let result = (result2 as f64).sqrt();
    println!("{}", result);
}
