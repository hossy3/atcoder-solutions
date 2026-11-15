use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            xy: [(i128, i128); n],
        }

        let mut count = 0isize;
        let mut map = BTreeMap::new();

        for &(x, y) in &xy {
            if x > 0 || (x == 0 && y > 0) {
                count += 1;
            }
            if x > 0 {
                let key = y * (1i128 << 64) / x;
                *map.entry(key).or_insert(0) -= 1;
            } else if x < 0 {
                let key = y * (1i128 << 64) / x;
                *map.entry(key).or_insert(0) += 1;
            }
        }

        let mut result = count.max(n as isize - count);
        for (_, &value) in &map {
            count += value;
            result = result.max(count).max(n as isize - count);
        }

        println!("{result}");
    }
}
