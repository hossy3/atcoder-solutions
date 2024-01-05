use itertools::Itertools;
use proconio::input;

type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        n: usize,
    }

    let zero = Mint::new(0);
    let one = Mint::new(1);
    let half = Mint::new(1) / Mint::new(2);

    // v[i][j]: (i + 1) 人の列で、j 番目の人が最後まで残る確率
    let mut v = vec![vec![]; n];
    v[0].push(one);

    for i in 1..n {
        // v[i][0] が分からないので、0, 1 開始それぞれ計算し、あとから混ぜる
        let mut v0 = vec![zero; i + 1];
        let mut v1 = vec![one; i + 1];

        for j in 1..=i {
            v0[j] = half * (v[i - 1][j - 1] + v0[j - 1]);
            v1[j] = half * (v[i - 1][j - 1] + v1[j - 1]);
        }

        // v0[0] (1-p) + v1[0] p = (v0[i] (1-p) + v1[i] p) / 2
        // (-v0[0] + v1[0] + half * v0[i] - half * v1[i]) = -v0[0] + half * v0[i]
        let p = (half * v0[i] - v0[0]) / (-v0[0] + v1[0] + half * (v0[i] - v1[i]));
        for j in 0..=i {
            v[i].push((one - p) * v0[j] + p * v1[j]);
        }
    }

    {
        let zero = num_rational::Rational64::new(0, 1);
        let one = num_rational::Rational64::new(1, 1);
        let half = num_rational::Rational64::new(1, 2);

        let mut v = vec![vec![]; n];
        v[0].push(one);

        for i in 1..n {
            let mut v0 = vec![zero; i + 1];
            let mut v1 = vec![one; i + 1];

            for j in 1..=i {
                v0[j] = half * (v[i - 1][j - 1] + v0[j - 1]);
                v1[j] = half * (v[i - 1][j - 1] + v1[j - 1]);
            }

            let p = (half * v0[i] - v0[0]) / (-v0[0] + v1[0] + half * (v0[i] - v1[i]));
            for j in 0..=i {
                v[i].push((one - p) * v0[j] + p * v1[j]);
            }
        }

        for i in 0..n {
            eprintln!("i: {:?}", v[i]);
        }
    }

    let result = v[n - 1].iter().join(" ");
    println!("{result}");
}
