use proconio::{input, marker::Chars};

fn rotate(s: &[Vec<char>]) -> Vec<Vec<char>> {
    let n = s.len();
    let mut s0 = vec![vec![' '; n]; n];
    for i in 0..n {
        for j in 0..n {
            s0[j][n - i - 1] = s[i][j];
        }
    }
    s0
}

fn f(s: &[Vec<char>], t: &[Vec<char>]) -> usize {
    let n = s.len();
    let mut count = 0usize;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] != t[i][j] {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
        t: [Chars; n],
    }
    let result = f(&s, &t);

    let s = rotate(&s);
    let result = result.min(f(&s, &t) + 1);

    let s = rotate(&s);
    let result = result.min(f(&s, &t) + 2);

    let s = rotate(&s);
    let result = result.min(f(&s, &t) + 3);

    println!("{result}");
}
