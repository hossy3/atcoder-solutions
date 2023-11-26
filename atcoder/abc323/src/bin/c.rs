use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
        s: [Chars; n],
    }

    let mut v = vec![];
    for (i, s) in s.iter().enumerate() {
        let mut score = i;
        for (j, x) in a.iter().enumerate() {
            if s[j] == 'o' {
                score += x;
            }
        }
        v.push(score);
    }

    let mut a0 = a.iter().enumerate().collect_vec();
    a0.sort_by(|a, b| b.1.cmp(a.1));

    let max_score = *v.iter().max().unwrap();

    for (i, &score) in v.iter().enumerate() {
        let mut count = 0;
        let mut score = score;
        for &(j, x) in &a0 {
            if score >= max_score {
                break;
            }
            if s[i][j] == 'x' {
                score += x;
                count += 1;
            }
        }
        println!("{count}");
    }
}
