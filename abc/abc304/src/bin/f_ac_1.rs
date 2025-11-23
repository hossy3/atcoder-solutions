use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    const MOD: usize = 998244353;
    let mut v = vec![];
    for i in 1..=(n / 2) {
        if n % i > 0 {
            continue;
        }

        let mut x = 1;
        let j = n / i;
        for i0 in 0..i {
            if (0..j).all(|j0| s[i * j0 + i0] == '#') {
                x = (x * 2) % MOD;
            }
        }
        let x0 = v
            .iter()
            .filter(|&(i0, _)| i % i0 == 0)
            .map(|&(_, x)| x)
            .sum::<usize>()
            % MOD;
        x = (x + MOD - x0) % MOD;
        v.push((i, x));
    }

    let result = v.iter().map(|&(_, x)| x).sum::<usize>() % MOD;
    println!("{}", result);
}
