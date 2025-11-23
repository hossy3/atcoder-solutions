use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            k: usize,
            a: [usize; n],
            b: [usize; n],
        }

        let mut ab = vec![];
        for i in 0..n {
            ab.push((a[i], b[i]));
        }
        ab.sort();

        let mut set = BTreeSet::new();
        let mut b_sum = 0;
        let mut result = usize::MAX;
        for i in 0..n {
            if set.len() == k {
                if let Some((b0, _)) = set.pop_last() {
                    b_sum -= b0;
                }
            }
            let (a, b) = ab[i];
            set.insert((b, i));
            b_sum += b;
            if set.len() == k {
                result = result.min(a * b_sum);
            }
        }

        println!("{result}");
    }
}
