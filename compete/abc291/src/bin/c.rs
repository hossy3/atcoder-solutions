use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    let mut set = HashSet::new();
    let mut x = 0i64;
    let mut y = 0i64;
    set.insert((x, y));

    for &c in &s {
        match c {
            'U' => {
                y -= 1;
            }
            'D' => {
                y += 1;
            }
            'L' => {
                x -= 1;
            }
            'R' => {
                x += 1;
            }
            _ => {}
        }
        if set.contains(&(x, y)) {
            println!("Yes");
            return;
        }
        set.insert((x, y));
    }
    println!("No");
}
