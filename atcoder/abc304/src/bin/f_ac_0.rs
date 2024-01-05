use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    const MOD: usize = 998244353;
    let mut v = vec![];
    for i in 1..=(n / 2) {
        if n % i == 0 {
            v.push((i, 0));
        }
    }

    for i in 0..(v.len()) {
        let mut x = 1;
        let m = v[i].0;
        let j = n / m;
        for i0 in 0..m {
            if (0..j).all(|j0| s[m * j0 + i0] == '#') {
                x = (x * 2) % MOD;
            }
        }
        v[i].1 = (v[i].1 + x) % MOD;
        for i0 in (i + 1)..(v.len()) {
            if v[i0].0 % v[i].0 == 0 {
                v[i0].1 = (v[i0].1 + MOD - v[i].1) % MOD;
            }
        }
    }

    let result = v.iter().map(|&(_, x)| x).sum::<usize>() % MOD;
    println!("{}", result);
}
