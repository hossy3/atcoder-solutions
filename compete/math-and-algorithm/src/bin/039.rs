use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        lrx: [(usize, usize, i64); q],
    }
    let mut imos = vec![0i64; n + 1];
    for &(l, r, x) in &lrx {
        imos[l - 1] += x;
        imos[r] -= x;
    }
    for i in 0..(n - 1) {
        imos[i + 1] += imos[i];
        let c = if imos[i] < imos[i + 1] {
            '<'
        } else if imos[i] == imos[i + 1] {
            '='
        } else {
            '>'
        };
        print!("{}", c);
    }
    println!();
}
