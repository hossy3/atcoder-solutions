use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut r: [usize; n],
        fs: [(Usize1, usize); m],
    }

    let mut result = 0;
    for (f, s) in fs {
        if r[f] >= s {
            r[f] -= s;
            result += 1;
        }
    }
    println!("{result}");
}
