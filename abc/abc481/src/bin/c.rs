use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        q: usize,
        s: [String; q],
    }

    let mut map = HashMap::new();
    for s in s {
        if !map.contains_key(&s) {
            map.insert(s.clone(), map.len() + 1);
        }
        let Some(result) = map.get(&s) else {
            unreachable!()
        };
        println!("{result}");
    }
}
