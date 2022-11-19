use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        _: usize,
        q: usize,
        tab: [(usize, usize, usize); q],
    }
    let mut s = HashSet::new();
    for &(t, a, b) in &tab {
        match t {
            1 => {
                s.insert((a, b));
            }
            2 => {
                s.remove(&(a, b));
            }
            3 => {
                let yes = s.contains(&(a, b)) && s.contains(&(b, a));
                println!("{}", if yes { "Yes" } else { "No" });
            }
            _ => {}
        }
    }
}
