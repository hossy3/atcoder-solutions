use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut result = t.len();
    for i in 0..=(s.len() - t.len()) {
        let mut count = 0usize;
        for j in 0..t.len() {
            if s[i + j] != t[j] {
                count += 1;
            }
        }
        result = result.min(count);
    }
    println!("{result}");
}
