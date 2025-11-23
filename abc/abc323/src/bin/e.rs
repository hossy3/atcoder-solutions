use proconio::input;

type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        n: usize,
        x: usize,
        t: [usize; n],
    }

    let mut v0 = vec![Mint::new(0); x + 2];
    let mut v = vec![vec![Mint::new(0); n]; x + 2];
    v0[0] = Mint::new(1);
    for i in 0..=x {
        if i > 0 {
            for j in 0..n {
                let p = v[i - 1][j];
                v[i][j] += p;
            }
        }

        let p = v0[i] / n;
        v0[i] = Mint::new(0);
        for j in 0..n {
            v[i][j] += p;
            v[(i + t[j]).min(x + 1)][j] -= p;
            v0[(i + t[j]).min(x + 1)] += p;
        }
    }

    let result = v[x][0];
    println!("{result}");
}
