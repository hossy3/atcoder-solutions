use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
    }
    let a = [
        (2, 1),
        (1, 2),
        (-1, 2),
        (-2, 1),
        (-2, -1),
        (-1, -2),
        (1, -2),
        (2, -1),
    ];
    let mut set = HashSet::new();
    for &(dx, dy) in &a {
        set.insert((x1 + dx, y1 + dy));
    }
    let f = || {
        for &(dx, dy) in &a {
            if set.contains(&(x2 + dx, y2 + dy)) {
                return true;
            }
        }
        false
    };
    let yes = f();
    println!("{}", if yes { "Yes" } else { "No" });
}
