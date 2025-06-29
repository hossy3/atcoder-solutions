use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    'outer: for _ in 0..t {
        input! {
            n: usize,
            s: [usize; n],
        }

        if s[0] >= s[n - 1] {
            println!("2");
            continue;
        }

        let mut set = BTreeSet::new();
        for &x in &s {
            if s[0] <= x && x <= s[n - 1] {
                set.insert(x);
            }
        }

        let mut cur = s[0];
        let mut count = 1;
        while cur != s[n - 1] {
            let Some(&next) = set.range(..=(cur * 2)).last() else {
                unreachable!();
            };
            if next == cur {
                println!("-1");
                continue 'outer;
            }
            cur = next;
            count += 1;
        }
        println!("{}", count);
    }
}
