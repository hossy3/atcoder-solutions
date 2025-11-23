use std::collections::BTreeMap;

use proconio::input;

type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        n: usize,
        a: [[usize; 6]; n],
    }

    let mut map = BTreeMap::new();
    for (i, v) in a.iter().enumerate() {
        for &x in v {
            map.entry(x).or_insert(vec![]).push(i);
        }
    }

    let sum = Mint::new(6).pow(n as u64); // 全部で何通りあるか
    let mut using = vec![0usize; n]; // それぞれ最初は 0 個使っている
    let mut rest = n; // 一度も使っていないサイコロの個数
    let mut cur = Mint::new(0); // 現在の組み合わせ個数
    let mut result = Mint::new(0);
    for (x, v) in map {
        if rest > 0 {
            for i in v {
                if using[i] == 0 {
                    rest -= 1;
                }
                using[i] += 1;
            }
            if rest == 0 {
                cur = Mint::new(1);
                for &y in &using {
                    cur *= y;
                }
                result = cur / sum * x;
            }
        } else {
            let prev = cur;
            for i in v {
                using[i] += 1;
                cur = cur / Mint::new(using[i] - 1) * Mint::new(using[i]);
            }
            result += (cur - prev) / sum * x;
        }
    }

    println!("{result}");
}
