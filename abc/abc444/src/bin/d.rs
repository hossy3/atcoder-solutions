use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let a_max = *a.iter().max().unwrap();
    let mut imos = vec![0isize; a_max + 1];
    for &x in &a {
        imos[0] += 1;
        imos[x] -= 1;
    }
    let mut v = vec![0isize; a_max];
    v[0] = imos[0];
    for i in 1..a_max {
        v[i] = v[i - 1] + imos[i];
    }

    // 繰り上がりする
    for i in 0..(a_max - 1) {
        v[i + 1] += v[i] / 10;
        v[i] %= 10;
    }
    let result = v.iter().rev().join("");
    println!("{result}");
}
