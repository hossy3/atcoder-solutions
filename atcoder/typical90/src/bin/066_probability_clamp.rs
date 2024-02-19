use proconio::input;

fn f(&(l0, r0): &(i64, i64), &(l1, r1): &(i64, i64)) -> f64 {
    let (n0, n1) = (r0 - l0 + 1, r1 - l1 + 1);
    let count: i64 = (l0..=r0).map(|i0| (i0 - l1).clamp(0, n1)).sum();
    let result = (count as f64) / ((n0 * n1) as f64);
    result
}

fn main() {
    input! {
        lr: [(i64, i64)],
    }
    let mut result = 0.0;
    for (i, lr0) in lr.iter().enumerate() {
        for lr1 in &lr[(i + 1)..] {
            result += f(lr0, lr1);
        }
    }
    println!("{result}");
}
