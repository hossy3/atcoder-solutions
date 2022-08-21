use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        t: usize,
        a: [usize; n - 1],
        xy: [(usize, usize); m],
    }
    let mut m = HashMap::new();
    for (x, y) in xy {
        m.insert(x, y);
    }
    let mut t: i64 = t as i64;
    for i in 0..(n - 1) {
        t -= a[i] as i64;
        if t <= 0 {
            break;
        }
        if let Some(&y) = m.get(&(i + 2)) {
            t += y as i64;
        }
    }
    let yes = t > 0;
    println!("{}", if yes { "Yes" } else { "No" });
}
