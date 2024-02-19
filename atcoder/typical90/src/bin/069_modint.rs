use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn f(n: u64, k: u32) -> Mint {
    if k == 1 {
        if n == 1 {
            Mint::new(1)
        } else {
            Mint::new(0)
        }
    } else {
        let k = Mint::new(k);
        if n == 1 {
            k
        } else if n == 2 {
            k * (k - 1)
        } else {
            (k * (k - 1)) * (k - 2).pow(n - 2)
        }
    }
}

fn main() {
    input! {
        n: u64,
        k: u32,
    }
    let count = f(n, k);
    println!("{count}");
}
