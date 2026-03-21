use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut v: [usize; n],
    }

    for _ in 0..q {
        input! {
            k: u8,
        }
        match k {
            1 => {
                input! {
                    a: Usize1,
                    b: Usize1,
                }
                v[b] += v[a];
                v[a] = 0;
            }
            2 => {
                input! {
                    c: Usize1,
                }
                println!("{}", v[c]);
            }
            _ => unreachable!(),
        }
    }
}
