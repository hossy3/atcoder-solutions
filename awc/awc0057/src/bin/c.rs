use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        n: usize,
        k: u64,
        mut l: [usize; n],
    }

    l.sort_unstable();
    let mut l = l.iter().map(|&x| Mint::new(x)).collect::<Vec<_>>();
    l[n - 1] *= Mint::new(2).pow(k);
    let result = l.iter().sum::<Mint>();
    println!("{result}");
}
