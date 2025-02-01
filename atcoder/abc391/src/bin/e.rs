use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        a: Chars,
    }

    // ふつうに解いたときの答えを求める
    let mut v = vec![];
    for &c in &a {
        let x: usize = if c == '1' { 1 } else { 0 };
        v.push(x);
    }
    let v = v;

    let mut v0 = v.clone();
    while v0.len() > 1 {
        let mut v1 = vec![];
        for i in 0..(v0.len() / 3) {
            let x = (v0[i * 3] + v0[i * 3 + 1] + v0[i * 3 + 2]) / 2;
            v1.push(x);
        }
        v0 = v1;
    }

    let old_result = v0[0];

    // 答えを変更するならどれだけ値を変えたくなるかを求める
    let mut v0 = vec![];
    for &x in &v {
        let x: usize = if x == old_result { 1 } else { 0 };
        v0.push(x);
    }
    while v0.len() > 1 {
        let mut v1 = vec![];
        for i in 0..(v0.len() / 3) {
            let mut v2 = vec![v0[i * 3], v0[i * 3 + 1], v0[i * 3 + 2]];
            v2.sort();
            let x = v2[0] + v2[1];
            v1.push(x);
        }
        v0 = v1;
    }

    let result = v0[0];
    println!("{result}");
}
