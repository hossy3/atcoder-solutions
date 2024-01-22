use proconio::input;

fn build_grundy() -> Vec<Vec<usize>> {
    const N: usize = 50;
    let b_max = N * (N + 1) / 2 + N + 1;
    let mut grundy = vec![vec![0; b_max]; N + 1];

    for w in 0..=N {
        for b in 0..b_max {
            let mut mex = vec![0usize; b_max];
            if w >= 1 && b + w < b_max {
                mex[grundy[w - 1][b + w]] = 1;
            }
            if b >= 2 {
                for k in 1..=(b / 2) {
                    mex[grundy[w][b - k]] = 1;
                }
            }
            grundy[w][b] = mex.iter().position(|&x| x == 0).unwrap();
        }
    }

    grundy
}

fn main() {
    input! {
        n: usize,
        w: [usize; n],
        b: [usize; n],
    }
    let grundy = build_grundy();
    let xor = std::iter::zip(w, b).fold(0, |acc, (w, b)| acc ^ grundy[w][b]);
    let yes = xor != 0;
    println!("{}", if yes { "First" } else { "Second" });
}
