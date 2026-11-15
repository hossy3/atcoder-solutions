use proconio::{input, marker::Usize1};

// a[0..i] の累積和を返す
fn f(i: usize, sum0: &[isize], sum1: &[isize], counts: &[usize]) -> isize {
    let j1 = counts.partition_point(|&count| count <= i) - 1;
    // eprintln!("i: {i}, j1: {j1}");
    let j0 = i - counts[j1];
    sum1[j1] + sum0[j0]
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [isize; n],
        lr: [(Usize1, Usize1); q],
    }

    // a の累積和
    let mut sum0 = vec![0isize; n + 1];
    for (i, &x) in a.iter().enumerate() {
        sum0[i + 1] = sum0[i] + x;
    }

    // sum0 の累積和
    let mut sum1 = vec![0isize; n + 1];
    for (i, &x) in sum0[1..].iter().enumerate() {
        sum1[i + 1] = sum1[i] + x;
    }

    // sum1 がそれぞれ何個扱うか
    let mut counts = vec![0usize; n + 1];
    for i in 0..n {
        counts[i + 1] = counts[i] + (i + 1);
    }

    // eprintln!("{:?} {:?} {:?}", &sum0, &sum1, &counts);

    for (l, r) in lr {
        let result = f(r + 1, &sum0, &sum1, &counts) - f(l, &sum0, &sum1, &counts);
        println!("{result}");
    }
}
