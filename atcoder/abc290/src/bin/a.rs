use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [Usize1; m],
    }
    let mut score = 0;
    for i in b {
        score += a[i];
    }
    println!("{}", score);
}
