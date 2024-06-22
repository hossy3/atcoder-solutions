use proconio::input;

type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        n: u64,
    }

    let k = n.ilog10() + 1;
    let k1 = 10u64.pow(k);
    let result = Mint::new(n) * ((Mint::new(k1).pow(n) - 1) / (Mint::new(k1) - 1));
    println!("{result}");
}
