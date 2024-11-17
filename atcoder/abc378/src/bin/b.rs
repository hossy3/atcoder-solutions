use proconio::input;

fn main() {
    input! {
        n: usize,
        qr: [(usize, usize); n],
        q: usize,
        td: [(usize, usize); q],
    }
    for (t, d) in td {
        let (q, r) = qr[t - 1];
        let mut x = (d / q) * q + r;
        if x < d {
            x += q;
        }
        println!("{x}");
    }
}
