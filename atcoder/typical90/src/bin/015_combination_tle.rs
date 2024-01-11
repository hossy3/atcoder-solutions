use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn combination(n: usize, r: usize) -> Mint {
    let r = r.min(n - r);
    (1..=r).fold(Mint::new(1), |acc, x| {
        acc * Mint::new(n - x + 1) / Mint::new(x)
    })
}

fn main() {
    input! {
        n: usize,
    }
    for k in 1..=n {
        let result = (1..=(1 + (n - 1) / k)).fold(Mint::new(0), |acc, i| {
            acc + combination(n - (k - 1) * (i - 1), i)
        });
        println!("{result}");
    }
}
