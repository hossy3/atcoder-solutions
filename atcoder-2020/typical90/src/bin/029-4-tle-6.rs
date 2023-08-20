use proconio::{input, marker::Usize1};

fn main() {
    input! {
        w: usize,
        lr: [(Usize1, Usize1)],
    }
    let mut v = vec![0usize; w];
    for &(l, r) in &lr {
        let height = v[l..=r].iter().max().unwrap() + 1;
        for h in v[l..=r].iter_mut() {
            *h = height;
        }
        println!("{}", height);
    }
}
