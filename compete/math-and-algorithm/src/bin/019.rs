use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }
    let mut v = [0usize; 3];
    for x in a {
        v[x] += 1;
    }
    let result: usize = v.iter().map(|x| x * (x - 1) / 2).sum();
    println!("{}", result);
}
