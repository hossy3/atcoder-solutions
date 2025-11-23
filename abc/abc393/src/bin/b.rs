use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut result = 0;
    for i in 0..s.len() {
        if s[i] != 'A' {
            continue;
        }
        for j in (i + 1)..s.len() {
            if s[j] != 'B' {
                continue;
            }
            let k = 2 * j - i;
            if k < s.len() && s[k] == 'C' {
                result += 1;
            }
        }
    }
    println!("{result}");
}
