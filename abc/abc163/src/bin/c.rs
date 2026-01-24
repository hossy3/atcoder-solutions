use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n - 1],
    }
    let mut v = vec![0usize; n];
    for i in a {
        v[i] += 1;
    }
    for x in v {
        println!("{x}");
    }
}
