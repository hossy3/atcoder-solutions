use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    let mut v = Vec::with_capacity(q + 2);
    v.push((usize::MAX, 1, 1)); // (value, prev_index, next_index)
    v.push((0, 0, 0));
    let mut map = HashMap::new();
    for (i, &(x, _, _)) in v.iter().enumerate() {
        map.insert(x, i); // value -> index
    }

    for q in 0..q {
        let q = q + 1; // query number
        input! {
            k: u8,
        }
        match k {
            1 => {
                input! {
                    x: usize,
                }
                let Some(&i) = map.get(&x) else {
                    unreachable!()
                };
                let j = v.len();
                let i0 = v[i].2;
                v.push((q, i, i0));
                map.insert(q, j);
                v[i0].1 = j;
                v[i].2 = j;
            }
            2 => {
                input! {
                    x: usize,
                    y: usize,
                }
                let Some(&i) = map.get(&x) else {
                    unreachable!()
                };
                let Some(&j) = map.get(&y) else {
                    unreachable!()
                };

                let mut z = i;
                while z != j && v[z].0 != usize::MAX {
                    z = v[z].2;
                }
                let (i, j) = if z == j { (i, j) } else { (j, i) }; // i -> j 順
                let mut sum = 0;
                let mut i0 = v[i].2;
                while i0 != j {
                    map.remove(&v[i0].0);
                    sum += v[i0].0;
                    i0 = v[i0].2;
                }
                v[i].2 = j;
                v[j].1 = i;
                println!("{sum}");
            }
            _ => unreachable!(),
        }
    }
}
