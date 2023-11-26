use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        ab: [(usize, usize); n],
    }

    let mut h = HashMap::new();
    h.insert(ab[0].0, vec!['H']);
    h.insert(ab[0].1, vec!['T']);
    for &(a, b) in &ab[1..] {
        let mut h0 = HashMap::new();
        for (k, v) in h {
            if !h0.contains_key(&(k + a)) {
                let mut v = v.clone();
                v.push('H');
                h0.insert(k + a, v);
            }
            if !h0.contains_key(&(k + b)) {
                let mut v = v.clone();
                v.push('T');
                h0.insert(k + b, v);
            }
        }
        h = h0;
    }
    
    if let Some(v) = h.get(&s) {
        println!("Yes");
        println!("{}", v.iter().join(""));
    } else {
        println!("No");
    }
}
