use proconio::{input, marker::Chars};

fn f(i: usize, j: usize, c: &[Vec<char>]) -> i64 {
    let (h, w) = (c.len(), c[0].len());
    let pos = |i, j| i * w + j;

    let mut score = -1;
    let mut stack = vec![];
    stack.push((i, j, 0, vec![false; w * h]));

    while let Some((i0, j0, len, visited)) = stack.pop() {
        if i == i0 && j == j0 && len > 0 {
            if len >= 4 {
                score = score.max(len);
            }
            continue;
        }

        let dir: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for &(di, dj) in &dir {
            let (i0, j0) = (i0.wrapping_add_signed(di), j0.wrapping_add_signed(dj));
            if i0 >= h || j0 >= w {
                continue;
            }
            let p = pos(i0, j0);
            if c[i0][j0] == '#' || visited[p] {
                continue;
            }
            let mut visited = visited.clone();
            visited[p] = true;
            stack.push((i0, j0, len + 1, visited));
        }
    }

    score
}

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let mut score = -1;
    for i in 0..h {
        for j in 0..w {
            score = score.max(f(i, j, &c));
        }
    }
    println!("{score}");
}
