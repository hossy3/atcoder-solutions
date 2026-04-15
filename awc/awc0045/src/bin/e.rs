use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        k: isize,
        a: [isize; n],
    }

    // 累積和
    let mut cum = vec![0; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + a[i];
    }

    // 区間最小値
    let mut set = BTreeSet::new();
    let mut v = vec![];
    for i in 0..w {
        set.insert((a[i], i));
    }
    v.push(set.iter().next().unwrap().0);
    for i in w..n {
        set.insert((a[i], i));
        set.remove(&(a[i - w], i - w));
        v.push(set.iter().next().unwrap().0);
    }

    let mut result = isize::MIN;
    for i in w..=n {
        let x = cum[i] - cum[i - w] + k * v[i - w];
        result = result.max(x);
    }
    println!("{result}");
}
