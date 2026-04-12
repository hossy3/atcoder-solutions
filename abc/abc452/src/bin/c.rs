use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        ab: [(usize, Usize1); n],
        m: usize,
        s: [Chars; m],
    }

    let ctoi = |c: char| (c as u8 - b'a') as usize;

    let mut v = vec![vec![[false; 26]; 10]; 11]; // [i][j][c] i 文字の文字列の j 番目に c が現れるか
    for s in &s {
        let i = s.len();
        for (j, &c) in s.iter().enumerate() {
            v[i][j][ctoi(c)] = true;
        }
    }

    for s0 in &s {
        let yes = if s0.len() == n {
            ab.iter()
                .enumerate()
                .all(|(i, &(a, b))| v[a][b][ctoi(s0[i])])
        } else {
            false
        };
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
