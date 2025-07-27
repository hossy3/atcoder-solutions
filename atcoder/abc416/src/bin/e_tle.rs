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

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, i64); m],
        k: usize,
        t: i64,
        mut d: [Usize1; k],
        q: usize,
    }

    let mut mat = vec![vec![i64::MAX / 2; n]; n];
    for i in 0..(d.len()) {
        mat[i][i] = 0;
    }
    for &(a, b, c) in &abc {
        mat[a][b] = mat[a][b].min(c);
        mat[b][a] = mat[b][a].min(c);
    }
    for i in 0..(d.len() - 1) {
        for j in (i + 1)..(d.len()) {
            let (i, j) = (d[i], d[j]);
            mat[i][j] = mat[i][j].min(t);
            mat[j][i] = mat[j][i].min(t);
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
                mat[x][y] = mat[x][y].min(t);
                mat[y][x] = mat[y][x].min(t);
            }
            2 => {
                input! {
                    x: Usize1,
                }
                if d.iter().all(|&y| x != y) {
                    for &y in &d {
                        mat[x][y] = mat[x][y].min(t);
                        mat[y][x] = mat[y][x].min(t);
                    }
                    d.push(x);
                }
            }
            3 => {
                let mut mat0 = vec![];
                for v in &mat {
                    mat0.push(v.clone());
                }
                let mat0 = floyd_warshall(mat0);
                let mut result = 0;
                for i in 0..(n - 1) {
                    for j in (i + 1)..n {
                        let x = mat0[i][j];
                        if x < i64::MAX / 2 {
                            result += x;
                        }
                    }
                }
                result *= 2;
                println!("{result}");
            }
            _ => unreachable!(),
        }
    }
}
