use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut dp = vec![(0, 0, 0); n + 1]; // R, P, S を出したときの最大勝利数, -1 はあり得ない
    for (i, &c) in s.iter().enumerate() {
        let (r0, p0, s0) = dp[i]; // 前回の手を出したときの最大勝利数
        let (mut r1, mut p1, mut s1) = (-1, -1, -1);
        match c {
            'R' => {
                // P 勝ち, R あいこ
                if r0 >= 0 {
                    p1 = p1.max(r0 + 1);
                }
                if p0 >= 0 {
                    r1 = r1.max(p0);
                }
                if s0 >= 0 {
                    r1 = r1.max(s0);
                    p1 = p1.max(s0 + 1);
                }
            }
            'P' => {
                // S 勝ち, P あいこ
                if r0 >= 0 {
                    p1 = p1.max(r0);
                    s1 = s1.max(r0 + 1);
                }
                if p0 >= 0 {
                    s1 = s1.max(p0 + 1);
                }
                if s0 >= 0 {
                    p1 = p1.max(s0);
                }
            }
            'S' => {
                // R 勝ち, S あいこ
                if r0 >= 0 {
                    s1 = s1.max(r0);
                }
                if p0 >= 0 {
                    r1 = r1.max(p0 + 1);
                    s1 = s1.max(p0);
                }
                if s0 >= 0 {
                    r1 = r1.max(s0 + 1);
                }
            }
            _ => unreachable!(),
        }

        dp[i + 1] = (r1, p1, s1);
    }

    let (rn, pn, sn) = dp[n];
    let result = rn.max(pn).max(sn);
    println!("{result}");
}
