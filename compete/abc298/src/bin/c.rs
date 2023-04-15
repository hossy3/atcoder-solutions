use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut box2num = vec![vec![]; n + 1];
    let mut num2box = vec![BTreeSet::new(); 200_001];
    for _ in 0..q {
        input! {
            t: usize,
            i: usize,
        }
        match t {
            1 => {
                input! {
                    j: usize,
                }
                box2num[j].push(i);
                num2box[i].insert(j);
            }
            2 => {
                box2num[i].sort();
                let result = box2num[i].iter().join(" ");
                println!("{}", result);
            }
            _ => {
                let result = num2box[i].iter().join(" ");
                println!("{}", result);
            }
        }
    }
}
