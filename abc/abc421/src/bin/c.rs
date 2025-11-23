use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut v = [vec![], vec![]];
    for (i, &c) in s.iter().enumerate() {
        match c {
            'A' => v[0].push(i),
            'B' => v[1].push(i),
            _ => unreachable!(),
        }
    }

    let mut result0 = 0usize;
    for i in 0..n {
        result0 += v[0][i].abs_diff(i * 2);
        result0 += v[1][i].abs_diff(i * 2 + 1);
    }

    let mut result1 = 0usize;
    for i in 0..n {
        result1 += v[0][i].abs_diff(i * 2 + 1);
        result1 += v[1][i].abs_diff(i * 2);
    }

    let result = result0.min(result1) / 2;
    println!("{result}");
}
