use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut start = 0usize;
    let mut v: Vec<usize> = (1..=n).collect();

    for _ in 0..q {
        input! {
            k: u8,
        }
        match k {
            1 => {
                input! {
                    p: Usize1,
                    x: usize,
                }
                let p = (start + p) % n;
                v[p] = x;
            }
            2 => {
                input! {
                    p: Usize1,
                }
                let p = (start + p) % n;
                println!("{}", v[p]);
            }
            3 => {
                input! {
                    k: usize,
                }
                start = (start + k) % n;
            }
            _ => unreachable!(),
        }
    }
}
