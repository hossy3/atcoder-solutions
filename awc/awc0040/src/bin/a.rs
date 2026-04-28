use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        p: [usize; n],
        tq: [(Usize1, usize); m],
    }

    let mut result = s;
    for (t, q) in tq {
        let x = p[t] * q;
        result += x;
        result -= x / 2;
    }
    println!("{result}");
}
