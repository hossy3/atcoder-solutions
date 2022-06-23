// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut plan0 = a[0];
    let mut plan1 = a[0];
    for i in 1..n {
        let x = plan0;
        plan0 = plan0.min(plan1) + a[i];
        plan1 = x;
    }
    plan0 = plan0.min(plan1);

    let mut plan2 = a[n - 1];
    let mut plan3 = a[n - 1];
    for i in 0..(n - 1) {
        let x = plan2;
        plan2 = plan2.min(plan3) + a[i];
        plan3 = x;
    }
    plan2 = plan2.min(plan3);

    println!("{}", plan0.min(plan2));
}
