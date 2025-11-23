use itertools::Itertools;
use proconio::input;
use std::{collections::{BTreeSet, BinaryHeap}, cmp::Reverse};

fn main() {
    input! {
        n: usize,
        td: [(usize, usize); n],
    }

    let mut se = td
        .iter()
        .map(|&(t, d)| (t, t + d))
        .into_iter()
        .collect_vec();
    se.sort();

    let mut queue = BinaryHeap::new();
    let mut i = 0;
    let mut cur = 1;
    let mut result = 0;
    loop {
        if queue.is_empty() {
            if i == n {
                break;
            }
            cur = se[i].0;
        }
        while i < n && se[i].0 == cur {
            queue.push(Reverse(se[i].1));
            i += 1;
        }
        while !queue.is_empty() && queue.peek().unwrap().0 < cur {
            queue.pop();
        }
        if let Some(_) = queue.pop() {
            result += 1;
        }
        cur += 1;
    }

    println!("{result}");
}
