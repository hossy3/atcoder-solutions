use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(i64, i64); n],
    }

    let min: i64 = lr.iter().map(|&(l, _)| l).sum();
    let max: i64 = lr.iter().map(|&(_, r)| r).sum();
    if min > 0 || max < 0 {
        println!("No");
        return;
    }

    println!("Yes");
    let mut diff = -min;
    let mut v = vec![];
    for (l, r) in lr {
        if diff == 0 {
            v.push(l);
        } else if r - l >= diff {
            v.push(l + diff);
            diff = 0;
        } else {
            v.push(r);
            diff -= r - l;
        }
    }
    let result = v.iter().join(" ");
    println!("{result}");
}
