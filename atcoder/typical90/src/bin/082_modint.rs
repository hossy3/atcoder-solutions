use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn g(k: usize) -> Mint {
    Mint::new(k) * Mint::new(k + 1) / 2
}

fn f(x: usize) -> Mint {
    let mut count = Mint::new(0);
    for i in 0..=18 {
        let y = 10usize.pow(i);
        if x < y {
            break;
        }
        count += g(x) - g(y - 1);
    }
    count
}

fn main() {
    input! {
        l: usize,
        r: usize,
    }
    let count = f(r) - f(l - 1);
    println!("{count}");
}
