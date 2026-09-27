use std::collections::BTreeMap;

use proconio::{input, marker::Usize1};
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut a = vec![0usize; n];
    let mut offset = 0usize; // この数だけ引く
    let mut map: BTreeMap<usize, usize> = BTreeMap::new();
    let mut result = 0usize;

    for _ in 0..q {
        input! {
            k: u8,
        }
        match k {
            1 => {
                input! {
                    x: Usize1,
                }

                if let Some(&v) = map.get(&a[x]) {
                    if v == 1 {
                        map.remove(&a[x]);
                    } else {
                        map.insert(a[x], v - 1);
                    }
                }

                let y = a[x].saturating_sub(offset);
                result = result ^ y;

                a[x] = a[x].max(offset) + 1;
                let y = a[x].saturating_sub(offset);

                result = result ^ y;
                *map.entry(a[x]).or_insert(0) += 1;

                // eprintln!("{:?}, {:?}, {}", &map, &a, result);
            }
            2 => {
                offset += 1;
                while let Some((&k, _)) = map.iter().next() {
                    if k > offset {
                        break;
                    }
                    map.pop_first();
                }

                result = 0;
                for (&k, &v) in &map {
                    if v % 2 == 1 {
                        result = result ^ k.saturating_sub(offset);
                    }
                }
            }
            _ => unreachable!(),
        }
        println!("{result}");
    }
}
