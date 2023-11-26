use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        xy: [(Usize1, usize); q],
    }

    let mut a = vec![0; n];

    let mut s0 = BTreeSet::new();
    for i in 0..(n - k) {
        s0.insert((0, i));
    }
    s0.insert((0, n)); // avoid empty

    let mut s1 = BTreeSet::new();
    for i in (n - k)..n {
        s1.insert((0, i));
    }
    let mut sum = 0usize;

    for &(x, y) in &xy {
        let y2 = a[x];
        a[x] = y;

        let &(y0, x0) = s0.iter().last().unwrap(); // large
        let &(y1, x1) = s1.iter().next().unwrap();

        if s0.contains(&(y2, x)) {
            s0.remove(&(y2, x));
            if (y, x) > (y1, x1) {
                s0.insert((y1, x1));
                sum -= y1;
                s1.remove(&(y1, x1));
                s1.insert((y, x));
                sum += y;
            } else {
                s0.insert((y, x));
            }
        } else {
            s1.remove(&(y2, x));
            sum -= y2;
            if (y, x) > (y0, x0) {
                s1.insert((y, x));
                sum += y;
            } else {
                s1.insert((y0, x0));
                sum += y0;
                s0.remove(&(y0, x0));
                s0.insert((y, x));
            }
        }

        println!("{}", sum);
    }
}
