use std::collections::HashMap;

use proconio::input;

type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        x: [usize; 3],
    }

    let mut dp: [HashMap<(usize, usize), Mint>; 3] =
        [HashMap::new(), HashMap::new(), HashMap::new()];
    dp[1].insert((0, 0), Mint::new(1));

    for xi in 0..(x[0] + x[1] + x[2]) {
        let mut dp0: [HashMap<(usize, usize), Mint>; 3] =
            [HashMap::new(), HashMap::new(), HashMap::new()];
        for (i, dp) in dp.iter().enumerate() {
            for (&(x0, x1), &count) in dp {
                let x2 = xi - x0 - x1;
                if i == 0 {
                    if x0 < x[0] {
                        *dp0[0].entry((x0 + 1, x1)).or_insert(Mint::new(0)) += count;
                    }
                    if x1 < x[1] {
                        *dp0[1].entry((x0, x1 + 1)).or_insert(Mint::new(0)) += count;
                    }
                } else if i == 1 {
                    if x0 < x[0] {
                        *dp0[0].entry((x0 + 1, x1)).or_insert(Mint::new(0)) += count;
                    }
                    if x1 < x[1] {
                        *dp0[1].entry((x0, x1 + 1)).or_insert(Mint::new(0)) += count;
                    }
                    if x2 < x[2] {
                        *dp0[2].entry((x0, x1)).or_insert(Mint::new(0)) += count;
                    }
                } else if i == 2 {
                    if x1 < x[1] {
                        *dp0[1].entry((x0, x1 + 1)).or_insert(Mint::new(0)) += count;
                    }
                    if x2 < x[2] {
                        *dp0[2].entry((x0, x1)).or_insert(Mint::new(0)) += count;
                    }
                }
            }
        }
        dp = dp0;
    }

    let result = dp[0].get(&(x[0], x[1])).unwrap_or(&Mint::new(0))
        + dp[1].get(&(x[0], x[1])).unwrap_or(&Mint::new(0))
        + dp[2].get(&(x[0], x[1])).unwrap_or(&Mint::new(0));
    println!("{result}");
}
