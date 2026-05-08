use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: [usize; n],
    }

    s.sort_unstable();
    let mut cur = Mint::new(0);
    for &x in &s[..(n - 1)] {
        cur += x;
    }
    cur += Mint::new(2).pow(k as u64) * s[n - 1];
    println!("{cur}");
}
