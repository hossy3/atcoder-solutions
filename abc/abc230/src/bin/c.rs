use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        p: i64,
        q: i64,
        r: i64,
        s: i64,
    }
    let mut v = vec![vec!['.'; (s - r + 1) as usize]; (q - p + 1) as usize];
    let k0 = (1 - a).max(1 - b).max(p - a).max(r - b);
    let k1 = (n - a).min(n - b).min(q - a).min(s - b);
    for k in k0..=k1 {
        let i = a + k - p;
        let j = b + k - r;
        v[i as usize][j as usize] = '#';
    }

    let k0 = (1 - a).max(b - n).max(p - a).max(b - s);
    let k1 = (n - a).min(b - 1).min(q - a).min(b - r);
    for k in k0..=k1 {
        let i = a + k - p;
        let j = b - k - r;
        v[i as usize][j as usize] = '#';
    }

    for v in v {
        let row = v.iter().join("");
        println!("{}", row);
    }
}
