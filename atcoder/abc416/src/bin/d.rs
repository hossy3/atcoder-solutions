use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            a: [usize; n],
            b: [usize; n],
        }

        // 貪欲にいけるはず
        let mut b_set = BTreeSet::new();
        for (i, &b) in b.iter().enumerate() {
            b_set.insert((b, i));
        }

        let mut result = 0usize;
        for &a in &a {
            let Some(&(b0, i0)) = b_set.iter().next() else {
                unreachable!()
            };
            if let Some(&(b1, i1)) = b_set.range((m - a, 0)..).next() {
                if (a + b1) % m < (a + b0) % m {
                    result += (a + b1) % m;
                    b_set.remove(&(b1, i1));
                    continue;
                }
            }
            result += (a + b0) % m;
            b_set.remove(&(b0, i0));
        }

        println!("{result}");
    }
}
