use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut map = BTreeMap::new();
    let mut counts = vec![1usize; n + 2];
    counts[0] = 0;
    counts[n + 1] = 0;

    for i in 0..=(n + 1) {
        map.insert(i, i); // 先頭の位置 (1-indexed) と、対応する色。ダミーで次の先頭終端も入れる
    }

    for _ in 0..q {
        input! {
            k: u8,
        }
        match k {
            1 => {
                input! {
                    x: usize,
                    c: usize,
                }
                let Some((&i1, &c1)) = map.range(..=x).last() else { unreachable!() };
                let Some((&i2, &c2)) = map.range((i1 + 1)..).next() else { unreachable!() };
                let Some((_, &c0)) = map.range(..i1).last() else { unreachable!() };
                if c == c0 && c == c2 {
                    map.remove(&i1);
                    map.remove(&i2);
                } else if c == c0 {
                    map.remove(&i1);
                } else if c == c2 {
                    map.insert(i1, c);
                    map.remove(&i2);
                } else {
                    map.insert(i1, c);
                }
                counts[c1] -= i2 - i1;
                counts[c] += i2 - i1;
            }
            2 => {
                input! {
                    x: usize,
                }
                let result = counts[x];
                println!("{result}");
            }
            _ => unreachable!(),
        }
    }
}
