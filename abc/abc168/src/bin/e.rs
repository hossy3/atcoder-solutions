use std::collections::HashMap;

use ac_library::ModInt1000000007;
use proconio::input;

type Mint = ModInt1000000007;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        n: usize,
        ab: [(isize, isize); n],
    }

    // ちょうど傾きが90度異なるものは仲が悪い。傾きを調べる
    let mut map = HashMap::new();
    for &(a, b) in &ab {
        let (a, b) = if a == 0 && b == 0 {
            (0, 0)
        } else if a == 0 {
            (0, 1)
        } else if b == 0 {
            (1, 0)
        } else {
            let x = gcd(a.abs() as usize, b.abs() as usize) as isize;
            let x = if a < 0 { -x } else { x };
            (a / x, b / x)
        };
        *map.entry((a, b)).or_insert(0u64) += 1;
    }

    let mut result = Mint::new(1);
    let m2 = Mint::new(2);
    for (&(a, b), &count) in &map {
        if (a, b) == (0, 0) {
            continue;
        }
        if let Some(&count0) = map.get(&(b, -a)) {
            result *= (m2.pow(count) - 1) + (m2.pow(count0) - 1) + 1;
        } else if !map.contains_key(&(-b, a)) {
            result *= m2.pow(count);
        }
    }
    result += *map.get(&(0, 0)).unwrap_or(&0); // (0, 0) はソロのみ大丈夫
    result -= 1; // 0匹を選ぶ場合はのぞく
    println!("{result}");
}
