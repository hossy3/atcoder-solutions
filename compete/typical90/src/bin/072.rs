use proconio::{input, marker::Chars};

fn init_f(h: usize, w: usize, c: &[Vec<char>]) -> Vec<Vec<bool>> {
    let mut a = vec![vec![false; w + 2]; h + 2];
    for i in 0..h {
        for j in 0..w {
            a[i + 1][j + 1] = c[i][j] == '.';
        }
    }
    a
}

fn search(i: usize, j: usize, f: &[Vec<bool>], i0: usize, j0: usize, n: i64, a: &mut [Vec<bool>]) -> i64 {
    if i0 == i && j0 == j && n > 3 {
        return n;
    }
    let mut score = -1;
    if f[i0][j0] && !a[i0][j0] {
        a[i0][j0] = true;
        score = score.max(search(i, j, f, i0 - 1, j0, n + 1, a));
        score = score.max(search(i, j, f, i0 + 1, j0, n + 1, a));
        score = score.max(search(i, j, f, i0, j0 - 1, n + 1, a));
        score = score.max(search(i, j, f, i0, j0 + 1, n + 1, a));
        a[i0][j0] = false;
    }
    score
}

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let f = init_f(h, w, &c);
    let mut score = -1;
    for i in 0..h {
        for j in 0..w {
            let mut a = vec![vec![false; w + 2]; h + 2];
            score = score.max(search(i, j, &f, i, j, 0, &mut a));
        }
    }
    println!("{}", score);
}
