use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; 4 * n - 1],
    }
    let mut v = vec![0; n];
    for i in a {
        v[i] += 1;
    }
    let result = v.iter().position(|&x| x == 3).unwrap() + 1;
    println!("{}", result);
}
