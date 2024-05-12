use ac_library::ModInt998244353;
use proconio::input;

type Mint = ModInt998244353;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut cum = vec![Mint::new(0); n + 1];
    for (i, &x) in a.iter().enumerate() {
        cum[i + 1] = cum[i] + x;
    }
    let mut result = Mint::new(0);
    for (i, &x) in a.iter().enumerate().rev() {
        let k = 10usize.pow(x.ilog10() + 1);
        result += cum[i] * k + x * i;
    }
    println!("{result}");
}
