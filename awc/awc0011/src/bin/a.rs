use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        s: usize,
        t: usize,
        pv: [(usize, usize); m],
    }

    let mut result = 0;
    for &(p, v) in &pv {
        if (s <= p && p <= t) || (t <= p && p <= s) {
            result += v;
        }
    }
    println!("{result}");
}
