use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut v = [0_usize; 100_001];
    for x in a {
        v[x] += 1;
    }
    let mut result = 0;
    for i in 1..50_000 {
        result += v[i] * v[100_000 - i];
    }
    if v[50_000] > 0 {
        result += v[50_000] * (v[50_000] - 1) / 2;
    }
    println!("{}", result);
}
