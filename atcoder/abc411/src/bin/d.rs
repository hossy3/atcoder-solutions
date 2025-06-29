use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut v = vec![0; n + 1];
    let mut buffers = vec![(0, vec![])]; // 1つ前の番号と追加文字列のペアを覚える
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
                    s: Chars,
                }
                buffers.push((v[p], s));
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

    let mut results = vec![];
    let mut i = v[0];
    while i > 0 {
        results.push(buffers[i].1.clone());
        i = buffers[i].0;
    }
    results.reverse();
    let result = results.iter().flat_map(|v| v).join("");
    println!("{result}");
}
