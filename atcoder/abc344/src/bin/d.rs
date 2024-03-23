use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: Chars,
        n: usize,
    }

    let nt = t.len();
    let mut dp = vec![usize::MAX; nt + 1];
    dp[0] = 0;

    for _ in 0..n {
        input! {
            x: usize,
            s: [Chars; x],
        }

        let prev = dp.clone();
        for (i, &x) in prev.iter().enumerate() {
            if x == usize::MAX {
                continue;
            }
            for s in &s {
                let ns = s.len();
                if i + ns > nt {
                    continue;
                }
                if t[i..(i + ns)] != *s {
                    continue;
                }
                dp[i + ns] = dp[i + ns].min(dp[i] + 1);
            }
        }
    }

    let result = dp[nt];
    if result < usize::MAX {
        println!("{result}");
    } else {
        println!("-1");
    }
}
