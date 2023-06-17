use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, i64); n],
    }
    const MIN: i64 = std::i64::MIN / 2;
    let mut v = vec![(MIN, MIN); n + 1]; // (good, bad)
    v[0].0 = 0;
    for (i, &(x, y)) in xy.iter().enumerate() {
        if x == 0 {
            v[i + 1].0 = v[i].0.max(v[i].0 + y).max(v[i].1 + y);
            v[i + 1].1 = v[i].1;
        } else {
            v[i + 1].0 = v[i].0;
            v[i + 1].1 = v[i].1.max(v[i].0 + y);
        }
    }
    let result = v[n].0.max(v[n].1);
    println!("{}", result);
}
