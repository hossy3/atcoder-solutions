use proconio::{input, marker::Chars};

// i 行目が条件を満たすときに最小値を返す
fn f_row(k: usize, s: &[Vec<char>], i: usize, mut result: usize) -> usize {
    let w = s[0].len();
    if w < k {
        return result;
    }

    'outer: for j in 0..=(w - k) {
        let mut x = 0;
        for j in j..(j + k) {
            match s[i][j] {
                '.' => {
                    x += 1;
                    if x >= result {
                        continue 'outer;
                    }
                }
                'o' => {}
                'x' => {
                    continue 'outer;
                }
                _ => unreachable!(),
            }
        }
        result = result.min(x);
    }

    result
}

// j 列目が条件を満たすときに最小値を返す
fn f_col(k: usize, s: &[Vec<char>], j: usize, mut result: usize) -> usize {
    let h = s.len();
    if h < k {
        return result;
    }

    'outer: for i in 0..=(h - k) {
        let mut x = 0;
        for i in i..(i + k) {
            match s[i][j] {
                '.' => {
                    x += 1;
                    if x >= result {
                        continue 'outer;
                    }
                }
                'o' => {}
                'x' => {
                    continue 'outer;
                }
                _ => unreachable!(),
            }
        }
        result = result.min(x);
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
