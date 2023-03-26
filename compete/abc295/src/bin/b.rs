use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        r: usize,
        c: usize,
        b: [Chars; r],
    }

    let mut s = BTreeSet::new();
    for y in 0..r {
        for x in 0..c {
            let chr = b[y][x];
            if chr >= '1' && chr <= '9' {
                let i = (chr as u8 - '1' as u8) as i64 + 1;
                for y0 in (-i)..=i {
                    for x0 in (-i)..=i {
                        if x0.abs() + y0.abs() <= i {
                            s.insert((x as i64 + x0, y as i64 + y0));
                        }
                    }
                }
            }
        }
    }

    for y in 0..r {
        for x in 0..c {
            let chr = if s.contains(&(x as i64, y as i64)) {
                '.'
            } else {
                b[y][x]
            };
            print!("{}", chr);
        }
        println!();
    }
}
