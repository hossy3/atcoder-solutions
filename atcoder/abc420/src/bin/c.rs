use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        mut b: [usize; n],
        cxv: [(char, Usize1, usize); q],
    }

    // 初期値
    let mut result = 0;
    for i in 0..n {
        result += a[i].min(b[i]);
    }

    for (c, x, v) in cxv {
        result -= a[x].min(b[x]);
        match c {
            'A' => a[x] = v,
            'B' => b[x] = v,
            _ => unreachable!(),
        }
        result += a[x].min(b[x]);
        println!("{result}");
    }
}
