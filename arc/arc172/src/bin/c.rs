use proconio::{input, marker::Chars};
use std::collections::HashSet;

type Mint0 = ac_library::ModInt998244353;
type Mint1 = ac_library::ModInt1000000007;

fn build_rolling_hash(nums: &[i32]) -> Vec<(Mint0, Mint1)> {
    let mut v = Vec::with_capacity(nums.len());
    v.push((Mint0::new(0), Mint1::new(0)));
    let mut k0 = Mint0::new(1);
    let mut k1 = Mint1::new(1);
    for &x in &nums[1..] {
        let y = if x > 0 {
            0
        } else if x < 0 {
            1
        } else {
            2
        };
        let x = *v.last().unwrap();
        let x0 = x.0 + Mint0::new(y) * k0;
        let x1 = x.1 + Mint1::new(y) * k1;
        v.push((x0, x1));
        k0 *= 3;
        k1 *= 3;
    }
    v
}

fn main() {
    input! {
        n: usize,
        c: Chars,
    }

    // 0 番目の人を含めた開票 A - B, 左端 0
    let mut v0 = vec![0i32; n + 1];
    for (i, &c) in c.iter().enumerate() {
        v0[i + 1] = v0[i]
            + match c {
                'A' => 1,
                'B' => -1,
                _ => unreachable!(),
            };
    }
    let h0 = build_rolling_hash(&v0);

    // 0 番目の人を含めない開票 A - B, 左端 0
    let mut v1 = vec![0i32; n + 1];
    for (i, &c) in c[1..].iter().enumerate() {
        v1[i + 1] = v1[i]
            + match c {
                'A' => 1,
                'B' => -1,
                _ => unreachable!(),
            };
    }
    let h1 = build_rolling_hash(&v1);

    let mut hash = HashSet::new();
    for i in 0..n {
        let h = (h1[i].0 + (h0[n].0 - h0[i].0), h1[i].1 + (h0[n].1 - h0[i].1));
        hash.insert(h);
    }

    let result = hash.len();
    println!("{result}");
}
