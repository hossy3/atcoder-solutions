use ac_library::FenwickTree;
use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn compress(a: &[usize]) -> Vec<usize> {
    let mut v = Vec::from(a);
    v.sort();
    v.dedup();

    let mut results = vec![];
    for x in a {
        results.push(v.binary_search(x).unwrap());
    }
    results
}

fn collect_ranges(k: i64, a: &[usize]) -> Vec<usize> {
    let n = a.len();
    let mut results = vec![];
    let mut tree = FenwickTree::new(n + 1, 0i64);
    let mut l = 0;
    let mut count = 0i64; // 転倒数
    for (_, &xr) in a.iter().enumerate() {
        tree.add(xr, 1);
        count += tree.sum((xr + 1)..);
        while count > k {
            let xl = a[l];
            tree.add(xl, -1);
            count -= tree.sum(..xl);
            l += 1;
        }
        results.push(l);
    }

    results
}

fn main() {
    input! {
        (n, k): (usize, i64),
        a: [usize; n],
    }

    let a = compress(&a);
    let ls = collect_ranges(k, &a);

    let mut dp = vec![Mint::new(0); n + 1];
    let mut cum = vec![Mint::new(0); n + 2];
    dp[0] = Mint::new(1);
    cum[1] = Mint::new(1);
    for r in 0..n {
        let l = ls[r];
        dp[r + 1] = cum[r + 1] - cum[l];
        cum[r + 2] = cum[r + 1] + dp[r + 1];
    }

    let result = dp[n];
    println!("{result}");
}
