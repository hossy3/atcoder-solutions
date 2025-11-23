use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut v = vec![0; n + 1];
    let mut buffers = vec![vec![]];
    for _ in 0..q {
        input! {
            k: u8,
        }
        match k {
            1 => {
                input! {
                    p: usize,
                }
                v[p] = v[0];
            }
            2 => {
                input! {
                    p: usize,
                    mut s: Chars,
                }
                let mut s0 = buffers[v[p]].clone();
                s0.append(&mut s);
                buffers.push(s0);
                v[p] = buffers.len() - 1;
            }
            3 => {
                input! {
                    p: usize,
                }
                v[0] = v[p];
            }
            _ => unreachable!(),
        }
    }

    let result = buffers[v[0]].iter().join("");
    println!("{result}");
}
