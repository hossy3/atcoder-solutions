use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut v = Vec::new();
    for _ in 0..n {
        input! {
            a: [usize],
        }
        v.push(a);
    }
    for _ in 0..q {
        input! {
            s: Usize1,
            t: Usize1,
        }
        println!("{}", v[s][t]);
    }
}
