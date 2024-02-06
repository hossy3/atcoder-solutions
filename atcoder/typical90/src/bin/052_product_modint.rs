use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        n: usize,
        a: [[usize; 6]; n],
    }
    let b: Vec<_> = a
        .iter()
        .map(|x| Mint::new(x.iter().sum::<usize>()))
        .collect();
    let score: Mint = b.iter().product();
    println!("{score}");
}
