use proconio::input;

fn main() {
    input! {
        ta: [(char, i64)],
    }

    const MOD: i64 = 10000;

    let mut x = 0;
    for (t, a) in &ta {
        match t {
            '+' => x += a,
            '-' => x -= a,
            _ => x *= a,
        }
        x = ((x % MOD) + MOD) % MOD;
        println!("{}", x);
    }
}
