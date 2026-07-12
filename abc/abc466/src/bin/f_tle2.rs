use std::collections::HashMap;

use proconio::input;

fn f(x: usize, a: &[usize]) -> usize {
    let mut map = HashMap::new();
    map.insert(x + 1, 1); // 余りが 0..x の数が 1ずつある
    let mut max = x + 1;
    for &a in a {
        if a >= max {
            continue;
        }
        max = max.min(a);

        let mut map0: HashMap<usize, usize> = HashMap::new();
        for (&k, &v) in &map {
            if k <= a {
                *map0.entry(k).or_insert(0) += v;
            } else {
                let v0 = k / a;
                *map0.entry(a).or_insert(0) += v * v0;
                let k0 = k % a;
                if k0 > 0 {
                    *map0.entry(k0).or_insert(0) += v;
                }
            }
        }

        map = map0;
        // eprintln!("{}: {:?}", a, &map);
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
