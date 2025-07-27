use proconio::{input, marker::Usize1};

fn floyd_warshall(mut d: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let n = d.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[i][j] = d[i][j].min(d[i][k] + d[k][j]);
            }
        }
    }
    d
}

const MAX: i64 = i64::MAX / 4;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, i64); m],
        k: usize,
        t: i64,
        d: [Usize1; k],
        q: usize,
    }

    let mut mat = vec![vec![MAX; n + 1]; n + 1]; // n: 空
    for i in 0..=n {
        mat[i][i] = 0;
    }
    for &(a, b, c) in &abc {
        mat[a][b] = mat[a][b].min(c);
        mat[b][a] = mat[b][a].min(c);
    }
    for &x in &d {
        mat[x][n] = t;
        mat[n][x] = 0;
    }
    mat = floyd_warshall(mat);
    let mut result = 0;
    for i in 0..n {
        for j in 0..n {
            let x = mat[i][j];
            if x < MAX {
                result += x;
            }
        }
    }

    for _ in 0..q {
        input! {
            tp: u8,
        }
        match tp {
            1 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                    t: i64,
                }

                if mat[x][y] < MAX {
                    result -= mat[x][y];
                }
                mat[x][y] = mat[x][y].min(t);
                result += mat[x][y];

                if mat[y][x] < MAX {
                    result -= mat[y][x];
                }
                mat[y][x] = mat[y][x].min(t);
                result += mat[y][x];

                for i in 0..=n {
                    for j in 0..=n {
                        if i < n && j < n && mat[i][j] < MAX {
                            result -= mat[i][j];
                        }
                        mat[i][j] = mat[i][j]
                            .min(mat[i][x] + mat[x][y] + mat[y][j])
                            .min(mat[i][y] + mat[y][x] + mat[x][j]);
                        if i < n && j < n && mat[i][j] < MAX {
                            result += mat[i][j];
                        }
                    }
                }
            }
            2 => {
                input! {
                    x: Usize1,
                }
                mat[x][n] = t;
                mat[n][x] = 0;
                for i in 0..=n {
                    for j in 0..=n {
                        if i < n && j < n && mat[i][j] < MAX {
                            result -= mat[i][j];
                        }
                        mat[i][j] = mat[i][j]
                            .min(mat[i][x] + mat[x][n] + mat[n][j])
                            .min(mat[i][n] + mat[n][x] + mat[x][j]);
                        if i < n && j < n && mat[i][j] < MAX {
                            result += mat[i][j];
                        }
                    }
                }
            }
            3 => {
                println!("{result}");
            }
            _ => unreachable!(),
        }
    }
}
