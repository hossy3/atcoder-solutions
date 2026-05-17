use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();

    let mut result = 0usize;
    for (i, &c) in s.iter().enumerate() {
        if c == 'C' {
            let x = (i + 1).min(n - i);
            result += x;
        }
    }
    println!("{result}");
}
