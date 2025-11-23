use std::collections::BTreeMap;

use proconio::input;

type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    let mut dp = vec![];
    let mut dp_sum = Mint::new(0);
    if k == 0 {
        dp.push(Mint::new(0));
    } else {
        dp.push(Mint::new(1));
        dp_sum += 1; // dp で採用しうる個数
    }

    let mut map = BTreeMap::new();
    if k == 0 {
        map.insert(0i64, Mint::new(0));
    } else  {
        map.insert(0i64, Mint::new(1)); // 合計 0 は 1通り
    }

    let mut offset = 0;
    for &x in &a {
        let mut result = Mint::new(0);
        offset += x;
        result += dp_sum; // i を採用する場合
        if let Some(&x) = map.get(&(k - offset)) {
            result -= x;
        }
        dp.push(result);

        if let Some(x) = map.get_mut(&(-offset)) {
            *x += result;
        } else {
            map.insert(-offset, result);
        }
        dp_sum += result;
    }

    let result = dp[n];
    println!("{result}");
}
