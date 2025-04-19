use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut b: [i64; n],
        mut w: [i64; m],
    }

    b.sort();
    w.sort();

    b.reverse();
    w.reverse();

    let mut b0 = vec![0i64; n + 1]; // 黒を i 個以上選ぶときのスコア最大値
    let mut w0 = vec![0i64; m + 1]; // 白を i 個ちょうど選ぶときのスコア最大値

    for i in 0..n {
        b0[i + 1] += b0[i] + b[i];
    }
    for i in 0..m {
        w0[i + 1] += w0[i] + w[i];
    }
    for i in (0..n).rev() {
        b0[i] = b0[i].max(b0[i + 1]);
    }

    let Some(result) = (0..=(n.min(m))).map(|i| b0[i] + w0[i]).max() else {
        unreachable!()
    };
    println!("{result}");
}
