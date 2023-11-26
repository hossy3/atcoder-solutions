use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut result = 0usize;
    for v0 in (0..n).permutations(2) {
        for v1 in (0..n).permutations(2) {
            let (i0, j0) = (v0[0], v1[0]);
            let (i1, j1) = (v0[1], v1[0]);
            let (i2, j2) = (v0[0], v1[1]);
            if s[i0][j0] == 'o' && s[i1][j1] == 'o' && s[i2][j2] == 'o' {
                result += 1;
            }
        }
    }
    println!("{result}");
}
