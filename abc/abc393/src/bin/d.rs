use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    // すべて左に寄せる
    let mut v = vec![];
    for (i, &c) in s.iter().enumerate() {
        if c == '1' {
            let x = (i - v.len()) as i64;
            v.push(x);
        }
    }

    let mut cur: i64 = v.iter().sum();
    let mut result = cur;
    let i_max = (n - v.len()) as i64;
    for i in 1..=i_max {
        let j = v.partition_point(|&x| x < i) as i64;
        cur += j * 2 - v.len() as i64;
        result = result.min(cur);
    }

    println!("{result}");
}
