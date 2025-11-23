use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();
    let mut result = 1usize;
    for i in 0..n {
        'outer: for j in (i + 1)..n {
            if j - i + 1 <= result {
                continue;
            }
            let m = (j - i + 1) / 2;
            for k in 0..m {
                if s[i + k] != s[j - k] {
                    continue 'outer;
                }
            }
            result = j - i + 1;
        }
    }
    println!("{result}");
}
