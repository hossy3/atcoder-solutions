use proconio::input;

fn f(&(l0, r0): &(usize, usize), &(l1, r1): &(usize, usize)) -> f64 {
    let mut count = 0;
    for i0 in l0..=r0 {
        if l1 < i0 {
            count += (i0 - l1).min(r1 - l1 + 1);
        }
    }
    let all = (r0 - l0 + 1) * (r1 - l1 + 1);
    let result = (count as f64) / (all as f64);
    result
}

fn main() {
    input! {
        lr: [(usize, usize)],
    }
    let mut result = 0.0;
    for (i, lr0) in lr.iter().enumerate() {
        for lr1 in &lr[(i + 1)..] {
            result += f(lr0, lr1);
        }
    }
    println!("{result}");
}
