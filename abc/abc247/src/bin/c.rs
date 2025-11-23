use itertools::Itertools;
use proconio::input;

fn f(n: usize) -> Vec<usize> {
    if n == 1 {
        vec![1]
    } else {
        let mut a = f(n - 1);
        let mut v = a.clone();
        v.push(n);
        v.append(&mut a);
        v
    }
}

fn main() {
    input! {
        n: usize,
    }
    let v = f(n);
    let result = v.iter().join(" ");
    println!("{}", result);
}
