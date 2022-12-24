use proconio::{input, marker::Usize1};

fn main() {
    input! {
        mut a: [usize],
        q: usize,
    }
    for _ in 0..q {
        input! {
            x: usize,
            k: Usize1,
        }
        if x == 1 {
            input! {
                y: usize,
            }
            a[k] = y;
        } else {
            println!("{}", a[k]);
        }
    }
}
