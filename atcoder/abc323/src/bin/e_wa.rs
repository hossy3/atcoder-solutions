use std::collections::BTreeMap;

use proconio::input;

type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        n: usize,
        x: usize,
        t: [usize; n],
    }

    let mut v = vec![vec![Mint::new(0); n]; x + 2];
    let mut queue = BTreeMap::new(); // (time, p)
    queue.insert(0, Mint::new(1));
    for i in 0..=n {
        if i > 0 {
            for j in 0..n {
                let x = v[i - 1][j];
                v[i][j] += x;
            }
        }
        if let Some((&time, &p)) = queue.first_key_value() {
            if time == i {
                queue.pop_first();
                let p0 = p / n;
                for j in 0..n {
                    v[i][j] += p0;
                    v[(i + t[j]).min(x + 1)][j] -= p0;
                    if i < n {
                        *queue.entry(i + t[i]).or_insert(Mint::new(0)) += p0;
                    }
                }
            }
        }
    }

    let result = v[x][0];
    println!("{result}");
}
