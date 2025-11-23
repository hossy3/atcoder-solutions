use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    const MOD: usize = 998244353;
    let mut mods = vec![1];
    for i in 0..q {
        let x = (mods[i] * 10) % MOD;
        mods.push(x);
    }

    let mut result = 1;
    let mut v = vec![1];
    let mut i = 0;

    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    x: usize,
                }
                result = (result * 10 + x) % MOD;
                v.push(x);
            }
            2 => {
                let j = v.len() - i - 1;
                result = (result + MOD - ((v[i] * mods[j]) % MOD)) % MOD;
                i += 1;
            }
            _ => {
                println!("{}", result);
            }
        }
    }
}
