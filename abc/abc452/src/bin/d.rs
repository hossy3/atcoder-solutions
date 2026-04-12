use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let ctoi = |c: char| (c as u8 - b'a') as usize;

    let mut positions = vec![vec![]; 26];
    for (i, &s0) in s.iter().enumerate() {
        positions[ctoi(s0)].push(i);
    }

    let mut result = 0usize;
    for i in 0..s.len() {
        let mut cur = i;
        for (j0, &t0) in t.iter().enumerate() {
            let j = positions[ctoi(t0)].partition_point(|&j| j < cur);
            if j < positions[ctoi(t0)].len() {
                cur = positions[ctoi(t0)][j];
                if j0 < t.len() - 1 {
                    cur += 1;
                }
            } else {
                cur = s.len();
                break;
            }
        }
        // eprintln!("{} {}", i, cur);
        result += cur.saturating_sub(i);
    }

    println!("{result}");
}
