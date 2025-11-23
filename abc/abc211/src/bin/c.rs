use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    const MOD: usize = 1_000_000_007;
    let mut v = vec![0usize; 9];
    v[0] = 1;
    for &c in &s {
        match c {
            'c' => v[1] = (v[0] + v[1]) % MOD,
            'h' => v[2] = (v[1] + v[2]) % MOD,
            'o' => v[3] = (v[2] + v[3]) % MOD,
            'k' => v[4] = (v[3] + v[4]) % MOD,
            'u' => v[5] = (v[4] + v[5]) % MOD,
            'd' => v[6] = (v[5] + v[6]) % MOD,
            'a' => v[7] = (v[6] + v[7]) % MOD,
            'i' => v[8] = (v[7] + v[8]) % MOD,
            _ => {}
        }
    }
    let result = v[8];
    println!("{}", result);
}
