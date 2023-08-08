use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        d: usize,
        s: [Chars; n],
    }
    let mut result = 0;
    let mut cur = 0;
    for i in 0..d {
        if (0..n).all(|j| s[j][i] == 'o') {
            cur += 1;
            result = result.max(cur);
        } else {
            cur = 0;
        }
    }
    println!("{}", result);
}
