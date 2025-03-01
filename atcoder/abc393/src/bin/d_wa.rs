use proconio::{input, marker::Chars};

// 1 の出現数の累積和を返す
fn f(s: &[char]) -> Vec<usize> {
    let mut v = vec![0usize; s.len() + 1];
    for (i, &c) in s.iter().enumerate() {
        let j = if c == '1' { 1 } else { 0 };
        v[i + 1] = v[i] + j;
    }
    v
}

// 1 の距離の累積和を返す
fn g(v: &[usize]) -> Vec<usize> {
    let mut cum = vec![0usize; v.len()];
    for i in 0..(v.len() - 1) {
        cum[i + 1] = cum[i] + v[i];
    }
    cum
}

fn main() {
    input! {
        n: usize,
        mut s: Chars,
    }

    let v0 = f(&s);
    let cum0 = g(&v0);

    s.reverse();
    let v1 = f(&s);
    let cum1 = g(&v1);

    let mut result = usize::MAX;
    for i0 in 0..n {
        let result0 = if i0 == 0 {
            0
        } else {
            let x = v0[i0];
            if x > 0 {
                cum0[i0] - (x * (x - 1)) / 2
            } else {
                cum0[i0]
            }
        };

        let i1 = n - i0 - 1;
        let result1 = if i1 == 0 {
            0
        } else {
            let x = v0[i1];
            if x > 0 {
                cum1[i1] - (x * (x - 1)) / 2
            } else {
                cum1[i1]
            }
        };
        result = result.min(result0 + result1);
    }

    println!("{result}");
}
