use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        x: usize,
        y: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }

    a.sort_unstable();
    b.sort_unstable();

    // a の累積和 (金額)
    let mut a_sum = vec![0usize; n + 1];
    for i in 0..n {
        a_sum[i + 1] = a_sum[i] + a[i];
    }

    // b の累積和 (Kドル使用数, 金額)
    let mut b_sum = vec![(0usize, 0usize); m + 1];
    for i in 0..m {
        b_sum[i + 1].0 = b_sum[i].0 + b[i].div_ceil(k);
        b_sum[i + 1].1 = b_sum[i].1 + b[i];
    }
    // eprintln!("{:?}", &b_sum);

    // b を $K 札で安いほうから買い占めて、a を安いほうから追加で購入したときの個数を調べる
    let mut result = 0usize;
    for i in 0..=m {
        if b_sum[i].0 > y {
            break;
        }
        let b_sum = b_sum[i].1;

        let j = a_sum.partition_point(|&a_sum| a_sum <= k * y + x - b_sum) - 1;
        // eprintln!("{}, {}, {}", i, j, b_sum);
        result = result.max(i + j);
    }

    println!("{result}");
}
