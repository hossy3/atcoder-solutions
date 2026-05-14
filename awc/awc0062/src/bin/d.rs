use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut result = 0usize;
    for d in 1..n {
        let mut v = vec![0usize]; // 値が異なる場所
        for i in 0..(n - d) {
            if s[i] != s[i + d] {
                v.push(i + 1);
            }
        }
        v.push(n - d + 1);

        for v in v.windows(3) {
            result += (v[2] - v[1]) * (v[1] - v[0]);
        }
    }

    println!("{result}");
}
