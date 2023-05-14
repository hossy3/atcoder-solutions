use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut x: Usize1,
        a: [Usize1; n],
    }
    let mut v = vec![false; n];
    while !v[x] {
        v[x] = true;
        x = a[x];
    }
    let result = v.iter().filter(|&&x| x).count();
    println!("{}", result);
}
