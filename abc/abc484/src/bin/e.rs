use ac_library::FenwickTree;
use proconio::input;

fn main() {
    input! {
        n: usize,
        l: isize,
        r: isize,
        a: [isize; n],
    }

    let mut sum = vec![0isize; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + a[i];
    }

    let mut sorted = sum.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    let mut map = std::collections::HashMap::new();
    for (i, &x) in sorted.iter().enumerate() {
        map.insert(x, i);
    }
    let sum0: Vec<_> = sum.iter().map(|&x| map[&x]).collect(); // 座標圧縮した位置

    let m = map.len();

    let mut fenwick = FenwickTree::new(m + 1, 0usize);
    let mut result = 0usize;

    // eprintln!("{:?}", &sorted);

    for i in (0..n).rev() {
        fenwick.add(sum0[i + 1], 1);

        // 今回の値を足したらどの範囲になるか
        let l0 = sorted.partition_point(|&x| x - sum[i] < l);
        let r0 = sorted.partition_point(|&x| x - sum[i] <= r);
        if l0 == r0 {
            continue;
        }
        result += fenwick.sum(l0..r0);
        // let ai = a[i];
        // eprintln!("a: {ai}, l0: {l0}, r0: {r0}, {result}");
    }

    println!("{result}");
}
