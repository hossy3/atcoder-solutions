use proconio::{input, marker::Usize1};

fn main() {
    input! {
        x: isize,
        n: usize,
        w: [isize; n],
        q: usize,
        p: [Usize1; q],
    }

    let mut b = vec![false; n];
    let mut result = x;
    for p in p {
        b[p] = !b[p];
        result += w[p] * if b[p] { 1 } else { -1 };
        println!("{result}");
    }
}
