use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize,
    }

    let mut x = Mint::new(a) * b * c;
    let mut result = Mint::new(1);
    for _ in 0..n {
        result *= x;
        x -= 1;
    }
    println!("{result}");
}
