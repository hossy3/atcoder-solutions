use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        xc: [(i64, usize); n],
    }

    let mut map = BTreeMap::<usize, (i64, i64)>::new();
    for &(x, c) in &xc {
        if let Some(&(x0, x1)) = map.get(&c) {
            map.insert(c, (x0.min(x), x1.max(x)));
        } else {
            map.insert(c, (x, x));
        }
    }
    map.insert(200_001, (0, 0));

    let mut dp = ((0, 0), (0, 0)); // ((pos0, cost0), (pos1, cost1))
    for (_, &(x0, x1)) in &map {
        let c0 = if dp.0 .0 <= x0 {
            (
                (dp.0 .1 + (x1 - dp.0 .0) + (x1 - x0)),
                (dp.0 .1 + x1 - dp.0 .0),
            )
        } else if x1 <= dp.0 .0 {
            (
                (dp.0 .1 + dp.0 .0 - x0),
                (dp.0 .1 + (dp.0 .0 - x0) + (x1 - x0)),
            )
        } else {
            (
                (dp.0 .1 + (x1 - dp.0 .0) + (x1 - x0)),
                (dp.0 .1 + (dp.0 .0 - x0) + (x1 - x0)),
            )
        };

        let c1 = if dp.1 .0 <= x0 {
            (
                (dp.1 .1 + (x1 - dp.1 .0) + (x1 - x0)),
                (dp.1 .1 + x1 - dp.1 .0),
            )
        } else if x1 <= dp.1 .0 {
            (
                (dp.1 .1 + dp.1 .0 - x0),
                (dp.1 .1 + (dp.1 .0 - x0) + (x1 - x0)),
            )
        } else {
            (
                (dp.1 .1 + (x1 - dp.1 .0) + (x1 - x0)),
                (dp.1 .1 + (dp.1 .0 - x0) + (x1 - x0)),
            )
        };

        dp = ((x0, c0.0.min(c1.0)), (x1, c0.1.min(c1.1)));
    }

    println!("{}", dp.0 .1);
}
