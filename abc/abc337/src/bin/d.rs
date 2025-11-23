use proconio::{input, marker::Chars};

// i 行目が条件を満たすときに最小値を返す
fn f_row(k: usize, s: &[Vec<char>], i: usize, mut result: usize) -> usize {
    let w = s[0].len();
    if w < k {
        return result;
    }

    let mut cum_o = vec![0usize; w + 1];
    let mut cum_x = vec![0usize; w + 1];
    for j in 0..w {
        cum_o[j + 1] += cum_o[j];
        if s[i][j] == 'o' {
            cum_o[j + 1] += 1;
        }

        cum_x[j + 1] += cum_x[j];
        if s[i][j] == 'x' {
            cum_x[j + 1] += 1;
        }
    }

    for j in 0..=(w - k) {
        if cum_x[j + k] == cum_x[j] {
            result = result.min(k - (cum_o[j + k] - cum_o[j]));
        }
    }

    result
}

// j 列目が条件を満たすときに最小値を返す
fn f_col(k: usize, s: &[Vec<char>], j: usize, mut result: usize) -> usize {
    let h = s.len();
    if h < k {
        return result;
    }

    let mut cum_o = vec![0usize; h + 1];
    let mut cum_x = vec![0usize; h + 1];
    for i in 0..h {
        cum_o[i + 1] += cum_o[i];
        if s[i][j] == 'o' {
            cum_o[i + 1] += 1;
        }

        cum_x[i + 1] += cum_x[i];
        if s[i][j] == 'x' {
            cum_x[i + 1] += 1;
        }
    }

    for i in 0..=(h - k) {
        if cum_x[i + k] == cum_x[i] {
            result = result.min(k - (cum_o[i + k] - cum_o[i]));
        }
    }

    result
}

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    }

    let mut result = usize::MAX;
    for i in 0..h {
        result = f_row(k, &s, i, result);
    }
    for j in 0..w {
        result = f_col(k, &s, j, result);
    }

    if result == usize::MAX {
        println!("-1");
    } else {
        println!("{result}");
    }
}
