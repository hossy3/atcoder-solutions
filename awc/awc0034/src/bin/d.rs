use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut v = vec![usize::MAX; n + 1]; // その回数の会議を終えられる最速の時刻
    v[0] = 0;
    for &(l, r) in lr.iter().sorted() {
        let l0 = v.partition_point(|&x| x <= l);
        v[l0] = v[l0].min(r);
        // eprintln!("l: {}, r: {}, l0: {}, {:?}", l, r, l0, &v);
    }
    let result = v.partition_point(|&x| x < usize::MAX) - 1;
    println!("{result}");
}
