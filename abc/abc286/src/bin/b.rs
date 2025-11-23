use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut v = Vec::with_capacity(n * 3 / 2);
    v.push(s[0]);
    for i in 1..n {
        if s[i - 1] == 'n' && s[i] == 'a' {
            v.push('y');
        } 
        v.push(s[i]);
    }
    for &v in &v {
        print!("{}", v);
    }
    println!();
}
