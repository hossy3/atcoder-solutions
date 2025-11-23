use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        s: usize,
    }
    let mut v = vec![Mint::new(0); s + 1];
    v[0] = Mint::new(1);
    for i in 3..=s {
        for j in 0..=(i - 3) {
            let x = v[j];
            v[i] += x;
        }
    }
    let result = v[s];
    println!("{result}");
}
