use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut v = vec![(0, 0); n + 1];
    for _ in 0..q {
        input! {
            k: usize,
            x: usize,
        }
        match k {
            1 => {
                input! {
                    y: usize,
                }
                v[x].1 = y;
                v[y].0 = x;
            }
            2 => {
                input! {
                    y: usize,
                }
                v[x].1 = 0;
                v[y].0 = 0;
            }
            _ => {
                let mut i = x;
                while v[i].0 != 0 {
                    i = v[i].0;
                }
                let mut results = vec![];
                while i != 0 {
                    results.push(i);
                    i = v[i].1;
                }
                let count = results.len();
                let result = results.iter().join(" ");
                println!("{} {}", count, result);
            }
        }
    }
}
