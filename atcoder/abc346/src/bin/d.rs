use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        c: [usize; n],
    }

    let mut cum0 = vec![0usize; n + 1]; // 010101... とするためのコスト累積和
    let mut cum1 = vec![0usize; n + 1]; // 101010... とするためのコスト累積和
    for (i, &ch) in s.iter().enumerate() {
        cum0[i + 1] = cum0[i];
        cum1[i + 1] = cum1[i];
        if (i % 2 == 0 && ch == '0') || (i % 2 == 1 && ch != '0') {
            cum1[i + 1] += c[i];
        } else {
            cum0[i + 1] += c[i];
        }
    }

    let mut result = usize::MAX;
    for i in 1..n {
        result = result.min(cum0[i] + cum1[n] - cum1[i]);
        result = result.min(cum1[i] + cum0[n] - cum0[i]);
    }
    println!("{result}");
}
