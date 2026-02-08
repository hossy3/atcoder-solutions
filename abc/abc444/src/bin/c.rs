use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();

    // 全部分割されている場合
    let t0 = a[0] + a[a.len() - 1];
    let b0 = (n % 2 == 0) && (0..(n / 2)).all(|i| a[i] + a[n - i - 1] == t0);

    // 最大の長さになる場合
    let t1 = a[a.len() - 1];
    let j = a.partition_point(|&x| x < t1);
    let b1 = (j % 2 == 0) && (0..(j / 2)).all(|i| a[i] + a[j - i - 1] == t1);

    if b0 && b1 {
        println!("{t1} {t0}");
    } else if b0 {
        println!("{t0}");
    } else if b1 {
        println!("{t1}");
    } else {
        unreachable!();
    }
}
