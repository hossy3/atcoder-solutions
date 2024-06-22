use itertools::Itertools;
use proconio::input;

fn f(level: u32, i: usize, j: usize, m: &mut [Vec<bool>]) {
    if level == 0 {
        m[i][j] = true;
    } else {
        let level = level - 1;
        let k = 3usize.pow(level);
        f(level, i, j, m);
        f(level, i, j + k, m);
        f(level, i, j + 2 * k, m);
        f(level, i + k, j, m);
        f(level, i + k, j + 2 * k, m);
        f(level, i + 2 * k, j, m);
        f(level, i + 2 * k, j + k, m);
        f(level, i + 2 * k, j + 2 * k, m);
    }
}

fn main() {
    input! {
        n: u32,
    }

    let k = 3usize.pow(n);
    let mut m = vec![vec![false; k]; k];
    f(n, 0, 0, &mut m);

    for v in m {
        let result = v.iter().map(|&b| if b { '#' } else { '.' }).join("");
        println!("{result}");
    }
}
