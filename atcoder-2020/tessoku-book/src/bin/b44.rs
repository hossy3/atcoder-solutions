use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        txy: [(usize, Usize1, Usize1)],
    }

    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push(i);
    }
    for (t, x, y) in txy {
        if t == 1 {
            v.swap(x, y);
        } else {
            let result = a[v[x]][y];
            println!("{}", result);
        }
    }
}
