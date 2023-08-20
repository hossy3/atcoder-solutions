use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n],
    }
    let mut imos = vec![0i64; t + 1];
    for &(l, r) in &lr {
        imos[l] += 1;
        imos[r] -= 1;
    }
    for i in 0..t {
        imos[i + 1] += imos[i];
        println!("{}", imos[i]);
    }
}
