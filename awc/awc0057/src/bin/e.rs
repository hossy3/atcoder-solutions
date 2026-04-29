use ac_library::FenwickTree;
use itertools::Itertools;
use proconio::input;

fn compress_coordinates(a: &[usize]) -> Vec<usize> {
    let mut sorted = a.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    let mut map = std::collections::HashMap::new();
    for (i, &x) in sorted.iter().enumerate() {
        map.insert(x, i);
    }
    a.iter().map(|&x| map[&x]).collect()
}

fn main() {
    input! {
        n: usize,
        pv: [(usize, usize); n],
    }

    let v = pv.iter().sorted().map(|&(_, v)| v).collect::<Vec<_>>();
    let v = compress_coordinates(&v);

    // 同じ位置、同じ速度は存在しないので、重複チェック不要
    let mut fenwick = FenwickTree::new(n, 0usize);
    let mut result = 0;
    for v in v {
        result += fenwick.sum(v..);
        fenwick.add(v, 1);
    }
    println!("{result}");
}
