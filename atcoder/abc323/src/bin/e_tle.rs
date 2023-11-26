use std::collections::BTreeMap;

use proconio::input;

type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        n: usize,
        x: usize,
        t: [usize; n],
    }
    let mut v = vec![vec![Mint::new(0); n]; x + 1];
    let mut queue = BTreeMap::new(); // (time, count)
    queue.insert(0, Mint::new(1));
    while let Some((time, count)) = queue.pop_first() {
        for i in 0..n {
            let mut count = count;
            for j in 0..t[i] {
                if time + j > x {
                    break;
                }
                v[time + j][i] += count;
                count *= n;
            }
            if time + t[i] <= x {
                *queue.entry(time + t[i]).or_insert(Mint::new(0)) += count / n;
            }
        }
    }

    let result = v[x][0] / Mint::new(n).pow((x + 1) as u64);
    println!("{result}");
}
