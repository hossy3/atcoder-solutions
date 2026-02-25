use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new(); // (次の数字候補, 個数)
    for x in a {
        let count = *map.get(&x).unwrap_or(&0_usize);
        let count0 = *map.get(&(x + 1)).unwrap_or(&0_usize);
        map.insert(x + 1, count0.max(count + 1));
    }
    let result = map.iter().map(|(_, &value)| value).max().unwrap();
    println!("{result}");
}
