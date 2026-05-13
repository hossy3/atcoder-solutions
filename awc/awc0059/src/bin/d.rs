use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut map = HashMap::new();
    map.insert(0, 1);
    let mut cur = 0isize;
    let mut result = 0usize;
    for c in s {
        match c {
            'V' => cur += 1,
            'F' => cur -= 1,
            _ => {}
        }
        if let Some(count) = map.get_mut(&cur) {
            result += *count;
            *count += 1;
        } else {
            map.insert(cur, 1);
        }
    }
    println!("{result}");
}
