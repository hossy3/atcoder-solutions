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
        lr: [(Usize1, Usize1); q],
    }

    const MOD: usize = 1000000007;
    const SIZE: usize = 26;

    let mut pow = Vec::with_capacity(n + 1);
    pow.push(1);
    for i in 0..n {
        pow.push((pow[i] * SIZE) % MOD);
    }

    let mut v = Vec::with_capacity(n + 1);
    v.push((0, 0));
    for i in 0..n {
        let l = (v[i].0 + (s[i] as u8 - b'a') as usize * pow[i + 1]) % MOD;
        let r = (v[i].1 + (s[n - i - 1] as u8 - b'a') as usize * pow[i + 1]) % MOD;
        v.push((l, r));
    }

    for (l, r) in lr {
        let l0 = ((v[r].0 + MOD - v[l].0) * pow[n - r - 1]) % MOD;
        let r0 = ((v[n - l - 1].1 + MOD - v[n - r - 1].1) * pow[l]) % MOD;
        let yes = l0 == r0;
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
