use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }
    let a = s.iter().position(|&x| x == 'A').unwrap();
    let b = s.iter().position(|&x| x == 'B').unwrap();
    let c = s.iter().position(|&x| x == 'C').unwrap();
    let result = a.max(b).max(c) + 1;
    println!("{}", result);
}
