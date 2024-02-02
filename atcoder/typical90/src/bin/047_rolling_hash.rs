use ac_library::ModInt998244353;
use itertools::Itertools;
use proconio::{input, marker::Chars};

type Mint = ModInt998244353;

fn build_nums(s: &[char]) -> Vec<usize> {
    let mut v = Vec::with_capacity(s.len());
    for x in s {
        let i = match x {
            'R' => 0,
            'G' => 1,
            'B' => 2,
            _ => unreachable!(),
        };
        v.push(i);
    }
    v
}

fn f(s: &[usize], t: &[usize]) -> usize {
    f_by_rolling_hash(s, t)
}

fn f_by_rolling_hash(s: &[usize], t: &[usize]) -> usize {
    let n = s.len();

    let mut k = vec![Mint::new(0); n + 1];
    k[0] = Mint::new(1);
    for i in 0..n {
        k[i + 1] = k[i] * 3;
    }

    let mut s0 = vec![Mint::new(0); n + 1];
    let mut t0 = vec![Mint::new(0); n + 1];
    for (i, &x) in s.iter().enumerate() {
        s0[i + 1] = s0[i] * 3 + x;
    }
    for (i, &x) in t.iter().enumerate() {
        t0[i + 1] = t0[i] * 3 + x;
    }

    let mut result = 0usize;
    for i in 0..n {
        if s0[n - i] == t0[n] - t0[i] * k[n - i] {
            result += 1;
        }
    }
    for i in 1..n {
        if t0[n - i] == s0[n] - s0[i] * k[n - i] {
            result += 1;
        }
    }

    result
}

fn main() {
    input! {
        _: usize,
        s: Chars,
        t: Chars,
    }

    let s = build_nums(&s);
    let t = build_nums(&t);

    let result = f(&s, &t.iter().map(|&i| [0, 2, 1][i]).collect_vec())
        + f(&s, &t.iter().map(|&i| [1, 0, 2][i]).collect_vec())
        + f(&s, &t.iter().map(|&i| [2, 1, 0][i]).collect_vec());
    println!("{result}");
}
