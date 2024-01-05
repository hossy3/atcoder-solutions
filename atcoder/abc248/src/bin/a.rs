use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut v = [false; 10];
    for c in s {
        v[c as usize - '0' as usize] = true;
    }
    let i = v.iter().position(|&x| !x).unwrap();
    println!("{}", i);
}
