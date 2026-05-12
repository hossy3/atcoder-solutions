use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut results = vec![0isize; n];

    for _ in 0..q {
        input! {
            k: u8,
        }
        match k {
            1 => {
                input! {
                    a: Usize1,
                    b: Usize1,
                    v: isize,
                }
                results[a] -= v;
                results[b] += v;
            }
            2 => {
                input! {
                    x: Usize1,
                    l: Usize1,
                    r: Usize1,
                }
                let result = (l..=r).filter(|&i| results[i] > results[x]).count();
                println!("{result}");
            }
            3 => {
                input! {
                    l: Usize1,
                    r: Usize1,
                    v: isize,
                }
                for i in l..=r {
                    results[i] += v;
                }
            }
            _ => unreachable!(),
        }
    }
}
