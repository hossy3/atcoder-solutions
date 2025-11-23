use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        abxy: [(Usize1, Usize1, i64, i64); m],
    }

    let mut v = vec![vec![]; n];
    for &(a, b, x, y) in &abxy {
        v[a].push((b, x, y));
        v[b].push((a, -x, -y));
    }

    let mut positions = vec![(0, 0); n];
    let mut visited = vec![false; n];
    visited[0] = true;
    let mut stack = vec![0];
    while let Some(i) = stack.pop() {
        for &(j, x, y) in &v[i] {
            if !visited[j] {
                visited[j] = true;
                positions[j] = (positions[i].0 + x, positions[i].1 + y);
                stack.push(j);
            }
        }
    }

    for (i, &(x, y)) in positions.iter().enumerate() {
        if visited[i] {
            println!("{x} {y}");
        } else {
            println!("undecidable");
        }
    }
}
