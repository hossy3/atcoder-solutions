use proconio::{input, marker::Chars};

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut m = vec![vec![vec![vec![usize::MAX; w]; h]; w]; h]; // 2点間の距離
    for i in 0..h {
        for j in 0..w {
            m[i][j][i][j] = 0;
        }
    }

    for _ in 0..(h * w) {
        for i0 in 0..h {
            for j0 in 0..w {
                if s[i0][j0] == '#' {
                    continue;
                }

                for i1 in 0..h {
                    for j1 in 0..w {
                        if s[i1][j1] == '#' || m[i0][j0][i1][j1] == usize::MAX {
                            continue;
                        }

                        for &(di, dj) in &DIRS {
                            let i2 = i1.wrapping_add_signed(di);
                            let j2 = j1.wrapping_add_signed(dj);
                            if i2 < h && j2 < w && s[i2][j2] == '.' {
                                m[i0][j0][i2][j2] = m[i0][j0][i2][j2].min(m[i0][j0][i1][j1] + 1);
                            }
                        }
                    }
                }
            }
        }
    }

    let mut result = 0;
    for i0 in 0..h {
        for j0 in 0..w {
            for i1 in 0..h {
                for j1 in 0..w {
                    let x = m[i0][j0][i1][j1];
                    if x != usize::MAX {
                        result = result.max(x);
                    }
                }
            }
        }
    }
    println!("{result}");
}
