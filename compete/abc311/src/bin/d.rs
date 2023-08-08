use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut mm = vec![vec![false; m]; n]; // moved
    let mut ms = vec![vec![false; m]; n]; // start
    let mut stack = vec![(1, 1)]; // (i, j)
    while let Some((i, j)) = stack.pop() {
        if ms[i][j] {
            continue;
        }
        ms[i][j] = true;
        mm[i][j] = true;

        for &(i0, j0) in &dirs {
            let mut i = i;
            let mut j = j;
            loop {
                let i0 = (i as i64 + i0) as usize;
                let j0 = (j as i64 + j0) as usize;
                if s[i0][j0] == '#' {
                    break;
                }
                i = i0;
                j = j0;
                mm[i][j] = true;
            }
            if !ms[i][j] {
                stack.push((i, j));
            }
        }
    }

    let result: usize = mm.iter().map(|v| v.iter().filter(|&&x| x).count()).sum();
    println!("{}", result);
}
