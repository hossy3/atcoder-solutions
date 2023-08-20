use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut map = HashMap::new();
    for x in a {
        *map.entry(x).or_insert(0usize) += 1;
    }
    let mut yes = true;
    for x in b {
        if let Some(&count) = map.get(&x) {
            if count == 1 {
                map.remove(&x);
            } else {
                map.insert(x, count - 1);
            }
        } else {
            yes = false;
            break;
        }
    }
    println!("{}", if yes { "Yes" } else { "No" });
}
