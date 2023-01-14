use proconio::{
    input,
    marker::{Chars, Usize1},
};

// string hash

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        abcd: [(Usize1, Usize1, Usize1, Usize1); q],
    }

    const MOD: usize = 1000000007;
    const SIZE: usize = 26;

    let mut pow = Vec::with_capacity(n + 1);
    pow.push(1);
    for i in 0..n {
        pow.push((pow[i] * SIZE) % MOD);
    }

    let mut v = Vec::with_capacity(n + 1);
    v.push(0);
    for i in 0..n {
        v.push((v[i] + (s[i] as u8 - b'a') as usize * pow[i + 1]) % MOD);
    }

    for &(a, b, c, d) in &abcd {
        let ab = (MOD + v[b + 1] - v[a]) % MOD;
        let cd = (MOD + v[d + 1] - v[c]) % MOD;
        let yes = if a >= c {
            ab == (cd * pow[a - c]) % MOD
        } else {
            (ab * pow[c - a]) % MOD == cd
        };
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
