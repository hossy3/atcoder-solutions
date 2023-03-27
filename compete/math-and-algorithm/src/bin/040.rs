use proconio::{input, marker::Usize1};

fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn main() {
    input! {
        n: usize,
        a: [usize; n - 1],
        m: usize,
        b: [Usize1; m],
    }
    let mut cum = a.clone();
    cum.insert(0, 0);
    for i in 0..(n - 1) {
        cum[i + 1] += cum[i];
    }
    let result: usize = b.windows(2).map(|x| abs_diff(cum[x[0]], cum[x[1]])).sum();
    println!("{}", result);
}
