use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        hw: [(usize, usize); n],
    }

    let mut hi: BTreeSet<_> = hw.iter().enumerate().map(|(i, &(h, _))| (h, i)).collect();
    let mut wi: BTreeSet<_> = hw.iter().enumerate().map(|(i, &(_, w))| (w, i)).collect();
    let mut h = h;
    let mut w = w;
    let mut results = vec![(0, 0); n];
    while hi.len() > 0 {
        if let Some(&(h0, i)) = hi.last() {
            if h0 == h {
                let (h0, w0) = hw[i];
                results[i] = (1, w - w0 + 1);
                hi.remove(&(h0, i));
                wi.remove(&(w0, i));
                w -= w0;
            }
        }
        if let Some(&(w0, i)) = wi.last() {
            if w0 == w {
                let (h0, w0) = hw[i];
                results[i] = (h - h0 + 1, 1);
                hi.remove(&(h0, i));
                wi.remove(&(w0, i));
                h -= h0;
            }
        }
    }

    for (x, y) in results {
        println!("{} {}", x, y);
    }
}
