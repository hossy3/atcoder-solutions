use std::collections::BTreeMap;

use proconio::input;

fn f(n: usize, a: usize, x: f64, y: f64, map: &mut BTreeMap<usize, f64>) -> f64 {
    if n == 0 {
        return 0.0;
    }
    if let Some(&x) = map.get(&n) {
        return x;
    }

    let r0 = f(n / a, a, x, y, map) + x;
    let mut r1 = y;
    for i in 2..=6 {
        r1 += f(n / i, a, x, y, map);
    }
    r1 /= 5.0;
    r1 += y;

    let z = r0.min(r1);
    map.insert(n, z);
    z
}

fn main() {
    input! {
        (n, a, x, y): (usize, usize, f64, f64),
    }

    let mut map = BTreeMap::new();
    let result = f(n, a, x, y, &mut map);
    println!("{result}");
}
