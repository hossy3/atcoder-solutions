use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 8],
    }
    let mut cols = [true; 8];
    let mut rows = [true; 8];
    for (r, s) in s.iter().enumerate() {
        for (c, &x) in s.iter().enumerate() {
            if x == '#' {
                rows[r] = false;
                cols[c] = false;
            }
        }
    }

    let mut result = 0;
    for r in 0..8 {
        if !rows[r] {
            continue;
        }
        for c in 0..8 {
            if !cols[c] {
                continue;
            }
            result += 1;
        }
    }
    println!("{result}");
}
