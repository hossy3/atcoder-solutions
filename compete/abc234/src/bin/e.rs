use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        x: i64,
    }

    let mut set = BTreeSet::new();

    for i in 1..=9 {
        set.insert(i);
        for j in (-9)..=9 {
            let mut x = i;
            let mut y = i;
            for _ in 0..17 {
                y += j;
                if y < 0 || y > 9 {
                    break;
                }
                x = x * 10 + y;
                set.insert(x);
            }
        }
    }

    let result = set.range(x..).next().unwrap();
    println!("{}", result);
}
