use proconio::input;

fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }

    const L: usize = 1500;
    let mut imos = vec![vec![0i64; L + 1]; L + 1];
    for &(a, b, c, d) in &abcd {
        imos[b][a] += 1;
        imos[b][c] -= 1;
        imos[d][a] -= 1;
        imos[d][c] += 1;
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

    let result: usize = imos
        .iter()
        .map(|v| v.iter().filter(|&&x| x != 0).count())
        .sum();
    println!("{}", result);
}
