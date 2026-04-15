use proconio::input;

fn main() {
    input! {
        n: usize,
        k: isize,
        mut d: [isize; n],
    }

    d.sort_unstable();
    let mut cum = vec![0; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + d[i];
    }

    let mut result = isize::MAX;
    for i in 0..n {
        // d[i] から +K 日間稼働する場合
        let r0 = d[i] + k - 1;
        let r0 = d.partition_point(|&x| x <= r0);
        let result0 = (-cum[i + 1] + d[i] * (i + 1) as isize)
            + (cum[n] - cum[r0] - (d[i] + k - 1) * (n - r0) as isize);
        result = result.min(result0);

        // d[i] から -K 日間稼働する場合
        let l0 = d[i] - k + 1;
        let l0 = d.partition_point(|&x| x <= l0);
        let result0 =
            (-cum[l0] + (d[i] - k + 1) * l0 as isize) + (cum[n] - cum[i] - d[i] * (n - i) as isize);
        result = result.min(result0);
    }

    println!("{result}");
}
