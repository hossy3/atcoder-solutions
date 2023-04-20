use proconio::{input, marker::Chars};

fn f(mut x: usize) -> Vec<usize> {
    let mut v = vec![];
    while x > 0 {
        v.push(x % 2);
        x /= 2;
    }
    v.reverse();
    v
}

fn g(v: &[usize]) -> usize {
    let mut x = 0;
    for &y in v {
        x = x * 2 + y;
    }
    x
}

fn main() {
    input! {
        _: usize,
        x: usize,
        s: Chars,
    }
    let mut v = f(x);
    for &c in &s {
        match c {
            'U' => {
                v.pop();
            }
            'L' => {
                v.push(0);
            }
            _ => {
                v.push(1);
            }
        }
    }
    let result = g(&v);
    println!("{}", result);
}
