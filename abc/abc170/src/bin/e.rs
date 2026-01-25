use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

const N: usize = 200_000;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut ab: [(usize, Usize1); n], // レート, 幼稚園
        cd: [(Usize1, Usize1); q], // 幼児番号, 幼稚園番号
    }

    let mut ks = vec![BTreeSet::new(); N]; // [幼稚園番号] = { (レート, 幼児番号), ... }
    for (i, &(a, b)) in ab.iter().enumerate() {
        ks[b].insert((a, i));
    }

    let mut scores = BTreeSet::new(); // { (レート, 幼稚園番号) }
    for (i, set) in ks.iter().enumerate() {
        if let Some((rate, _)) = set.last() {
            scores.insert((*rate, i));
        }
    }

    for &(c, d) in &cd {
        let (a, b) = ab[c]; // (レート, 以前の幼稚園番号)
        ab[c] = (a, d);

        if let Some((rate, _)) = ks[b].last() {
            scores.remove(&(*rate, b));
        }
        ks[b].remove(&(a, c));
        if let Some((rate, _)) = ks[b].last() {
            scores.insert((*rate, b));
        }

        if let Some((rate, _)) = ks[d].last() {
            scores.remove(&(*rate, d));
        }
        ks[d].insert((a, c));
        if let Some((rate, _)) = ks[d].last() {
            scores.insert((*rate, d));
        }

        let result = (*scores.iter().next().unwrap()).0;
        println!("{result}");

        // eprintln!("{:?}", &ks[0..=2]);
        // eprintln!("{:?}", &scores);
    }
}
