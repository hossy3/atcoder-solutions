use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize,
        k: usize,
    }

    const MOD: usize = 998_244_353;

    let mut dp = Vec::<HashMap::<(usize, usize, usize, usize), usize>>::new();
    dp.push(HashMap::<(usize, usize, usize, usize), usize>::new()); // r - k
    dp.push(HashMap::<(usize, usize, usize, usize), usize>::new()); // g - k
    dp.push(HashMap::<(usize, usize, usize, usize), usize>::new()); // b
    dp.push(HashMap::<(usize, usize, usize, usize), usize>::new()); // k
    if r > k {
        dp[0].insert((1, 0, 0, 0), 1);
    }
    if g > k {
        dp[1].insert((0, 1, 0, 0), 1);
    }
    if b > 0 {
        dp[2].insert((0, 0, 1, 0), 1);
    }
    if k > 0 {
        dp[3].insert((0, 0, 0, 1), 1);
    }
    for _ in 1..(r + g + b - k) {
        let mut dp_next = dp.clone();
        for i in 0..4 {
            for (&(r0, g0, b0, k0), &n) in &dp[i] {
                if r - r0 > k {
                    if let Some(x) = dp_next[i].get_mut(&(r0 + 1, g0, b0, k0)) {
                        *x = (*x + n) % MOD;
                    } else {
                        dp_next[i].insert((r0 + 1, g0, b0, k0), n);
                    }
                } 
                if g - g0 > k && i != 0 {
                    if let Some(x) = dp_next[i].get_mut(&(r0, g0 + 1, b0, k0)) {
                        *x = (*x + n) % MOD;
                    } else {
                        dp_next[i].insert((r0, g0 + 1, b0, k0), n);
                    }
                } 
                if b - b0 > 0 {
                    if let Some(x) = dp_next[i].get_mut(&(r0, g0, b0 + 1, k0)) {
                        *x = (*x + n) % MOD;
                    } else {
                        dp_next[i].insert((r0, g0, b0 + 1, k0), n);
                    }
                } 
                if k - k0 > 0 {
                    if let Some(x) = dp_next[i].get_mut(&(r0, g0, b0, k0 + 1)) {
                        *x = (*x + n) % MOD;
                    } else {
                        dp_next[i].insert((r0, g0, b0, k0 + 1), n);
                    }
                } 
            }
        }
        dp = dp_next;
    }
    
    let mut score = 0;
    for m in dp {
        score += m.get(&(r - k, g - k, b, k)).unwrap_or(&0);
    }
    println!("{}", score);
}
