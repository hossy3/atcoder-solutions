use std::collections::BinaryHeap;

use ordered_float::OrderedFloat;
use proconio::input;

fn f(a: &[f64], mut k: usize, x: usize) -> f64 {
    let mut queue = BinaryHeap::new();
    for &a in a {
        queue.push((OrderedFloat(a), 1usize));
    }

    while k > 0 {
        let Some((OrderedFloat(a), count)) = queue.pop() else {
            unreachable!();
        };
        if count > k {
            queue.push((OrderedFloat(a / 2.0), k * 2));
            queue.push((OrderedFloat(a), count - k));
            k -= k;
        } else {
            queue.push((OrderedFloat(a / 2.0), count * 2));
            k -= count;
        }
    }

    // x番目に大きなものを探す
    let mut x0 = 0usize;
    while let Some((OrderedFloat(a), count)) = queue.pop() {
        x0 += count;
        if x0 >= x {
            return a;
        }
    }
    unreachable!();
}

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            k: usize,
            x: usize,
            a: [f64; n],
        }
        let result = f(&a, k, x);
        println!("{result}");
    }
}
