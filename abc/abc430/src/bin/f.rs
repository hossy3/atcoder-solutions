use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
        }

        // 左に残り何個連続した L, R があるか
        let mut l_rep_to_l = vec![0usize; n];
        let mut r_rep_to_l = vec![0usize; n];
        for i in 0..(n - 1) {
            l_rep_to_l[i + 1] = if s[i] == 'L' { l_rep_to_l[i] + 1 } else { 0 };
            r_rep_to_l[i + 1] = if s[i] == 'R' { r_rep_to_l[i] + 1 } else { 0 };
        }

        // 右に残り何個連続した L, R があるか
        let mut l_rep_to_r = vec![0usize; n];
        let mut r_rep_to_r = vec![0usize; n];
        for i in (0..(n - 1)).rev() {
            l_rep_to_r[i] = if s[i] == 'L' {
                l_rep_to_r[i + 1] + 1
            } else {
                0
            };
            r_rep_to_r[i] = if s[i] == 'R' {
                r_rep_to_r[i + 1] + 1
            } else {
                0
            };
        }

        // いもす法で区間内を +1 する
        let mut imos = vec![0isize; n + 1];
        for i in 0..n {
            let l0 = r_rep_to_l[i] + l_rep_to_r[i];
            let r0 = n - (l_rep_to_l[i] + r_rep_to_r[i]);
            imos[l0] += 1;
            imos[r0] -= 1;
        }

        let mut results = vec![0isize; n + 1];
        for i in 0..n {
            results[i + 1] = results[i] + imos[i];
        }
        println!("{}", results[1..].iter().join(" "));
    }
}
