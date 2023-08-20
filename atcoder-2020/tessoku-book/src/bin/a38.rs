use proconio::{input, marker::Usize1};

fn main() {
    input! {
        d: usize,
        n: usize,
        lrh: [(Usize1, Usize1, usize); n],
    }

    let mut a = vec![24; d];
    for &(l, r, h) in &lrh {
        for i in l..=r {
            a[i] = a[i].min(h);
        }
    }
    let result: usize = a.iter().sum();
    println!("{}", result);
}
