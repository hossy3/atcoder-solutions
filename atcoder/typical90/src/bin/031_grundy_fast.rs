use proconio::input;

fn build_grundy() -> Vec<Vec<usize>> {
    const N: usize = 50;
    let mut grundy = vec![vec![]; N + 1];
    grundy[N] = vec![0usize; N + 1];
    for i in (1..=N).rev() {
        let len = grundy[i].len() + i;
        grundy[i - 1] = vec![0usize; len];
    }

    for w in 0..=N {
        let len = grundy[w].len();
        for b in 0..len {
            let mut mex = vec![false; len + 1];
            if w >= 1 {
                mex[grundy[w - 1][b + w]] = true;
            }
            if b >= 2 {
                for k in 1..=(b / 2) {
                    mex[grundy[w][b - k]] = true;
                }
            }
            grundy[w][b] = mex.iter().position(|&x| !x).unwrap();
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
