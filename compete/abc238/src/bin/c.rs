use proconio::input;

const MOD: u128 = 998244353;

fn f(n: u128) -> u128 {
    for k in (1..=17).rev() {
        let m = 10u128.pow(k);
        if n >= m {
            return (((n - m + 1) * (n - m + 2)) / 2 + f(m - 1)) % MOD;
        }
    }
    ((n * (n + 1)) / 2) % MOD
}

#[test]
fn test_func() {
    assert_eq!(f(16), 73);
    assert_eq!(f(238), 13870);
    assert_eq!(f(999999999999999999), 762062362);
}

fn main() {
    input! {
        n: u128,
    }
    let result = f(n);
    println!("{}", result);
}
