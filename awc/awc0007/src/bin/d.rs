use proconio::{input, marker::Usize1};

/// (r1 - r0) * (c1 - c0) の大きさの長方形を重ねた時に、各グリッドに何枚あるかを imos 2D で求める
fn count_grid_by_imos_2d(n: usize, rc: &[(usize, usize, usize, usize)]) -> Vec<Vec<usize>> {
    let mut imos = vec![vec![0isize; n + 1]; n + 1];
    for &(r0, c0, r1, c1) in rc {
        imos[r0][c0] += 1;
        imos[r1 + 1][c0] -= 1;
        imos[r0][c1 + 1] -= 1;
        imos[r1 + 1][c1 + 1] += 1;
    }

    let mut counts = vec![vec![0isize; n + 1]; n + 1];
    for x in 0..n {
        for y in 0..n {
            counts[x + 1][y + 1] = counts[x][y + 1] + counts[x + 1][y] - counts[x][y] + imos[x][y];
        }
    }

    let mut results = vec![vec![0; n]; n];
    for x in 0..n {
        for y in 0..n {
            results[x][y] = counts[x + 1][y + 1] as usize;
        }
    }
    results
}

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        rca: [(Usize1, Usize1, Usize1, Usize1); a],
        rcb: [(Usize1, Usize1, Usize1, Usize1); b],
    }
    let ma = count_grid_by_imos_2d(n, &rca);
    let mb = count_grid_by_imos_2d(n, &rcb);
    let result = (0..n)
        .map(|x| (0..n).filter(|&y| ma[x][y] > 0 && mb[x][y] > 0).count())
        .sum::<usize>();
    println!("{result}");
}
