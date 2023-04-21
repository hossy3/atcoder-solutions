use proconio::{input, marker::Usize1};

// Floyd-Warshall Algorithm

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, usize); m],
    }

    let mut d = vec![vec![2_000_000_000; n]; n];
    for &(a, b, c) in &abc {
        d[a][b] = c;
        d[b][a] = c;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[i][j] = d[i][j].min(d[i][k] + d[k][j]);
            }
        }
    }

    let mut result = 0;
    for &(a, b, c) in &abc {
        if (0..n).any(|i| d[a][i] + d[i][b] <= c) {
            result += 1;
        }
    }
    println!("{}", result);
}
