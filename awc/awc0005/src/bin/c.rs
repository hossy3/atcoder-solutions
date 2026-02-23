use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    let mut queue = BinaryHeap::new();
    for (i, &x) in a.iter().enumerate() {
        queue.push((x, i));
    }

    let mut result = 0;
    while let Some((x, i)) = queue.pop() {
        if a[i] != x {
            continue;
        }
        if i > 0 && a[i - 1] + k < x {
            let j = i - 1;
            let y = x - k;
            result += y - a[j];
            a[j] = y;
            queue.push((y, j))
        }
        if i < n - 1 && a[i + 1] + k < x {
            let j = i + 1;
            let y = x - k;
            result += y - a[j];
            a[j] = y;
            queue.push((y, j))
        }
    }

    println!("{result}");
}
