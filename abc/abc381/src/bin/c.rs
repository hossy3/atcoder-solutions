use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut v1 = vec![0usize; n]; // 左から数えて 1 が連続する数
    for i in 0..n {
        if s[i] == '1' {
            if i == 0 {
                v1[i] = 1;
            } else {
                v1[i] = v1[i - 1] + 1;
            }
        }
    }

    let mut v2 = vec![0usize; n]; // 右から数えて 2 が連続する数
    for i in (0..n).rev() {
        if s[i] == '2' {
            if i == n - 1 {
                v2[i] = 1;
            } else {
                v2[i] = v2[i + 1] + 1;
            }
        }
    }

    let mut result = 1; // 最小の答え。S は / を 1 個以上含むため
    for i in 1..(n - 1) {
        if s[i] == '/' {
            let x = v1[i - 1].min(v2[i + 1]) * 2 + 1;
            result = result.max(x);
        }
    }
    println!("{result}");
}
