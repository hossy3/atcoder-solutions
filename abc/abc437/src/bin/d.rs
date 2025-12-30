use proconio::input;

type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        mut b: [usize; m],
    }

    b.sort();
    let mut cum = vec![0; m + 1];
    for (i, &x) in b.iter().enumerate() {
        cum[i + 1] = cum[i] + x;
    }

    let mut result = Mint::new(0);
    for &a in &a {
        let i = b.partition_point(|&b| b < a);
        result += a * i;
        result -= cum[i];

        result += cum[m];
        result -= cum[i];
        result -= a * (m - i);
    }
    println!("{result}");
}
