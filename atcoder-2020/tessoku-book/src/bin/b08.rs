use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    }

    const L: usize = 1500;
    let mut imos = vec![vec![0i64; L + 1]; L + 1];
    for &(x, y) in &xy {
        imos[y][x] += 1;
    }
    for x in 0..=L {
        for y in 0..L {
            imos[y + 1][x] += imos[y][x];
        }
    }
    for y in 0..=L {
        for x in 0..L {
            imos[y][x + 1] += imos[y][x];
        }
    }

    for &(a, b, c, d) in &abcd {
        let result = (imos[b - 1][a - 1] + imos[d][c]) - (imos[b - 1][c] + imos[d][a - 1]);
        println!("{}", result);
    }
}
