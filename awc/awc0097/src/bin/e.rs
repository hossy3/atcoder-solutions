use proconio::input;

/// longest increasing subsequence
fn lis(a: &[usize]) -> Vec<usize> {
    let mut v = vec![0; a.len()];
    let mut p = vec![];

    for (i, &x) in a.iter().enumerate() {
        let j = p.partition_point(|&y| y <= x); // 同値も OK
        if j == p.len() {
            p.push(x);
        } else if p[j] > x {
            p[j] = x;
        }
        v[i] = j + 1;
    }
    v
}

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [usize; n],
    }

    let mut h0 = vec![0usize; n];
    for (i, &h) in h.iter().enumerate() {
        h0[i] = h + i * (k - 1);
    }

    if h0[n - 1] < h0[0] {
        println!("-1");
        return;
    }

    let h0 = h0
        .iter()
        .filter(|&&x| h0[0] <= x && x <= h0[n - 1])
        .copied()
        .collect::<Vec<_>>();

    let v = lis(&h0);
    let result = n - v[v.len() - 1];
    // eprintln!("{:?}", &v);
    println!("{result}");
}
