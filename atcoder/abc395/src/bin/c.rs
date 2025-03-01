use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut map = HashMap::new();
    let mut result = isize::MAX;
    for (i, &x) in a.iter().enumerate() {
        if let Some(j) = map.get(&x) {
            result = result.min((i - j + 1) as isize);
        }
        map.insert(x, i);
    }
    if result == isize::MAX {
        result = -1;
    }
    println!("{result}");
}
