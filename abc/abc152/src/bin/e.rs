use std::collections::{HashMap, HashSet};

use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn prime_division(mut n: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let mut k = 2;
    while k * k <= n {
        let mut count = 0;
        while n % k == 0 {
            count += 1;
            n /= k;
        }
        if count > 0 {
            result.push((k, count));
        }
        k += if k == 2 { 1 } else { 2 };
    }
    if n > 1 {
        result.push((n, 1));
    }
    result
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    // 最小公倍数を求める
    let mut set = HashSet::new();
    for &x in &a {
        set.insert(x);
    }

    let mut map = HashMap::new();
    for &x in &set {
        let v = prime_division(x);
        for (k, x) in v {
            if let Some(&y) = map.get(&k) {
                map.insert(k, x.max(y));
            } else {
                map.insert(k, x);
            }
        }
    }

    let mut lcm = Mint::new(1);
    for (&k, &v) in &map {
        lcm *= k.pow(v as u32);
    }

    let result = a.iter().map(|&x| lcm / x).sum::<Mint>();
    println!("{result}");
}
