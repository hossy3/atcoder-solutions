use proconio::{input, marker::Usize1};

fn f(n: usize, rc: &[(usize, usize, usize, usize)]) -> Vec<Vec<bool>> {
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

    let mut results = vec![vec![false; n]; n];
    for x in 0..n {
        for y in 0..n {
            results[x][y] = counts[x + 1][y + 1] > 0;
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
    let ma = f(n, &rca);
    let mb = f(n, &rcb);
    let result = (0..n)
        .map(|x| (0..n).filter(|&y| ma[x][y] && mb[x][y]).count())
        .sum::<usize>();
    println!("{result}");
}
