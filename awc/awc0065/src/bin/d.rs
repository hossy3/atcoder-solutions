use proconio::input;

fn f(x: usize, y: usize) -> (usize, usize) {
    (x + y, 1000 + x - y)
}

fn main() {
    input! {
        n: usize,
        m: usize,
        xyc: [(usize, usize, usize); n],
        pqk: [(usize, usize, usize); m],
    }

    const N: usize = 2001;

    let mut acc = vec![vec![0usize; N + 1]; N + 1];
    for (x, y, c) in xyc {
        let (x, y) = f(x, y);
        acc[x + 1][y + 1] += c;
    }

    for i in 0..=N {
        for j in 1..=N {
            acc[i][j] += acc[i][j - 1];
        }
    }
    for i in 1..=N {
        for j in 0..=N {
            acc[i][j] += acc[i - 1][j];
        }
    }

    for (p, q, k) in pqk {
        let (p, q) = f(p, q);
        let (p0, q0) = (p.saturating_sub(k), q.saturating_sub(k));
        let (p1, q1) = ((p + k + 1).min(N), (q + k + 1).min(N));
        let result = (acc[p1][q1] + acc[p0][q0]) - (acc[p0][q1] + acc[p1][q0]);
        println!("{result}");
    }
}
