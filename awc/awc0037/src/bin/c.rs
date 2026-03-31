use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut cur = Mint::new(1);
    for i in 0..k {
        cur *= Mint::new(a[i]);
    }

    let mut result = cur;
    for i in k..n {
        cur *= Mint::new(a[i]);
        cur /= Mint::new(a[i - k]);
        result += cur;
    }
    println!("{result}");
}
