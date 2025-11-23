use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    'outer: for i in 1..=(n - 1) {
        for j in 0..(n - i) {
            if s[j] == s[j + i] {
                println!("{}", j);
                continue 'outer;
            }
        }
        println!("{}", n - i);
    }
}
