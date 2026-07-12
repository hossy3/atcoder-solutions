use std::collections::BTreeMap;

use proconio::input;

fn f(x: usize, a: &[usize]) -> usize {
    let mut map = BTreeMap::new();
    map.insert(x + 1, 1); // 余りが 0..x の数が 1ずつある
    for &a in a {
        let Some((&max, _)) = map.iter().last() else {
            unreachable!()
        };
        if a >= max {
            continue;
        }

        while let Some((&k, &v)) = map.iter().last() {
            if k <= a {
                break;
            }
            map.pop_last();

            let v0 = k / a;
            *map.entry(a).or_insert(0) += v * v0;
            let k0 = k % a;
            if k0 > 0 {
                *map.entry(k0).or_insert(0) += v;
            }
        }
    }

    let result = map.values().sum::<usize>() - 1;
    result
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            x: usize,
            a: [usize; n],
        }
        let result = f(x, &a);
        println!("{result}");
    }
}
