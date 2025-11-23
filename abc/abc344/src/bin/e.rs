use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    }

    let mut map = HashMap::new();

    // build linked-list
    map.insert(0, (a[a.len() - 1], a[0]));
    map.insert(a[0], (0usize, 0usize));
    for v in a.windows(2) {
        map.insert(v[1], (v[0], 0));
        map.get_mut(&v[0]).unwrap().1 = v[1];
    }

    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {
                    (x, y): (usize, usize),
                }
                let &(_, r) = map.get(&x).unwrap();
                map.get_mut(&x).unwrap().1 = y;
                map.get_mut(&r).unwrap().0 = y;
                map.insert(y, (x, r));
            }
            2 => {
                input! {
                    x: usize,
                }
                let &(l, r) = map.get(&x).unwrap();
                map.get_mut(&l).unwrap().1 = r;
                map.get_mut(&r).unwrap().0 = l;
                map.remove(&x);
            }
            _ => unreachable!(),
        }
    }

    let mut results = vec![];
    let mut x = map.get(&0).unwrap().1;
    while x != 0 {
        results.push(x);
        x = map.get(&x).unwrap().1;
    }

    let result = results.iter().join(" ");
    println!("{result}");
}
