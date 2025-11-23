use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        w: usize,
        b: usize,
    }

    let s: Vec<_> = "wbwbwwbwbwbw".chars().collect();
    let mut set = HashSet::new();
    for i in 0..12 {
        let mut w = 0usize;
        let mut b = 0usize;
        for j in 0..300 {
            if s[(i + j) % 12] == 'w' {
                w += 1;
            } else {
                b += 1;
            }
            set.insert((w, b));
        }
    }

    let yes = set.contains(&(w, b));
    println!("{}", if yes { "Yes" } else { "No" });
}
