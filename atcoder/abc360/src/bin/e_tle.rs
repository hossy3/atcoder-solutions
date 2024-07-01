use proconio::input;

type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let p0 = Mint::new(1) / Mint::new(n); // 狙った場所を引く
    let p1 = Mint::new(1) - p0; // それ以外を引く

    let p0011 = p0 * p0 + p1 * p1;
    let p0000 = p0 * p0 + p0 * p0;

    let mut p = Mint::new(1); // 左端にある確率
    for _ in 0..k {
        p = p * p0011 + (Mint::new(1) - p) * p0000;
    }

    let mut result = Mint::new(0);
    
    for i in 1..=n {
        let mut p2 = if i == 1 {
            p
        } else {
            (Mint::new(1) - p) / Mint::new(n - 1)
        };
        p2 *= Mint::new(i);
        result += p2;
    }
    println!("{result}");
}
