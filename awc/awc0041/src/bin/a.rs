use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ck: [(usize, usize); n],
        p: [Usize1; m],
    }

    let mut result = 0;
    for p in p {
        if ck[p].1 > 0 {
            result += ck[p].0;
            ck[p].1 -= 1;
        }
    }
    println!("{result}");
}
