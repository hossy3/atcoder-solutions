use proconio::{input, marker::Chars};

fn c2i(c: char) -> usize {
    (c as u8 - b'a') as usize
}

fn main() {
    const N: usize = 26;

    input! {
        n: usize,
        l: usize,
        r: usize,
        s: Chars,
    }

    let mut cum = vec![[0usize; N]; n + 1];
    for (i, &c) in s.iter().enumerate() {
        cum[i + 1][c2i(c)] = 1;
    }
    for i in 0..n {
        for j in 0..N {
            cum[i + 1][j] += cum[i][j];
        }
    }

    let mut result = 0usize;
    for (i, &c) in s.iter().enumerate() {
        let j = c2i(c);
        let il = (i + l).min(n);
        let ir = (i + r + 1).min(n);
        let x = cum[ir][j] - cum[il][j];
        result += x;
    }
    println!("{result}");
}
