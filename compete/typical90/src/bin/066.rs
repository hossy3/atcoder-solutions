use proconio::input;

fn f(lr0: &(usize, usize), lr1: &(usize, usize)) -> f64 {
    let mut count = 0;
    for i in lr0.0..=lr0.1 {
        if lr1.0 < i {
            count += (i - lr1.0).min(lr1.1 - lr1.0 + 1);
        }
    }
    let result = (count as f64) / (((lr0.1 - lr0.0 + 1) * (lr1.1 - lr1.0 + 1)) as f64);
    result
}

fn main() {
    input! {
        lr: [(usize, usize)],
    }
    let mut result = 0.0;
    for i in 0..(lr.len() - 1) {
        for j in (i + 1)..lr.len() {
            result += f(&lr[i], &lr[j]);
        }
    }
    println!("{}", result);
}
