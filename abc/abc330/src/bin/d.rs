use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut i2count = vec![0; n];
    let mut j2count = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'o' {
                i2count[i] += 1;
                j2count[j] += 1;
            }
        }
    }

    let mut result = 0usize;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] != 'o' {
                continue;
            }
            result += (i2count[i] - 1) * (j2count[j] - 1);
        }
    }
    println!("{result}");
}
