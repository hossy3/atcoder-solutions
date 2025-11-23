use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [usize; n],
    }

    let mut map = BTreeMap::new();
    map.insert(0usize, usize::MAX);

    let mut result = usize::MAX;

    for x in x {
        let Some((&x0, &dist0)) = map.range(..=x).last() else {
            unreachable!()
        };

        map.insert(x0, dist0.min(x - x0));
        result -= dist0;
        result += dist0.min(x - x0);

        if let Some((&x1, &dist1)) = map.range(x..).next() {
            map.insert(x, (x - x0).min(x1 - x));
            result += (x - x0).min(x1 - x);

            map.insert(x1, dist1.min(x1 - x));
            result -= dist1;
            result += dist1.min(x1 - x);
        } else {
            map.insert(x, x - x0);
            result += x - x0;
        }

        println!("{result}");
    }
}
