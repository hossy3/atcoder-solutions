use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        q: usize,
        k: [usize; q],
    }

    let mut results = vec![];
    for k in k {
        let mut x = (k - 1) / s.len();
        let y = (k - 1) % s.len();
        let mut b = false; // 反転するか
        while x > 0 {
            if x % 2 == 1 {
                b = !b;
            }
            x /= 2;
        }

        let mut c = s[y];
        if b {
            if 'a' <= c && c <= 'z' {
                c = (c as u8 + 'A' as u8 - 'a' as u8) as char;
            } else {
                c = (c as u8 + 'a' as u8 - 'A' as u8) as char;
            }
        }
        results.push(c);
    }

    let result = results.iter().join(" ");
    println!("{result}");
}
