use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1],
    }
    let mut v = vec![a.len(); n];
    for a in a {
        v[a] -= 1;
    }
    for v in v {
        println!("{}", v);
    }
}
