use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut v = vec![false; n];
    for _ in 0..k {
        input! {
            a: [Usize1],
        }
        for i in a {
            v[i] = true;
        }
    }
    let result = v.iter().filter(|&&b| !b).count();
    println!("{result}");
}
