use std::collections::{HashMap, HashSet, VecDeque};

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn get_pos(a: &[Vec<char>], c: char) -> (usize, usize) {
    for (i, a) in a.iter().enumerate() {
        for (j, &x) in a.iter().enumerate() {
            if x == c {
                return (i, j);
            }
        }
    }
    unreachable!()
}

fn g(a: &[Vec<char>], (sr, sc): (usize, usize), rc: &[(usize, usize)]) -> Vec<usize> {
    let h = a.len();
    let w = a[0].len();

    let mut v = vec![usize::MAX; rc.len()];
    let mut map = HashMap::new();
    for (i, &(r, c)) in rc.iter().enumerate() {
        map.insert((r, c), i);
    }

    let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    let mut visited = vec![vec![false; w]; h];
    visited[sr][sc] = true;

    let mut queue = VecDeque::new();
    queue.push_back((0, (sr, sc)));

    while let Some((step, (r, c))) = queue.pop_front() {
        if let Some(&i) = map.get(&(r, c)) {
            v[i] = step;
        }

        for &(dr, dc) in &dirs {
            let r = r.wrapping_add_signed(dr);
            let c = c.wrapping_add_signed(dc);
            if r >= h || c >= w {
                continue;
            }

            if a[r][c] == '#' {
                continue;
            }
            if !visited[r][c] {
                visited[r][c] = true;
                queue.push_back((step + 1, (r, c)));
            }
        }
    }

    v
}

fn f(a: &[Vec<char>], rce: &[(usize, usize, usize)]) -> bool {
    let n = rce.len();

    let (dr, sc) = get_pos(&a, 'S');
    let (tr, tc) = get_pos(&a, 'T');

    let mut graph = vec![vec![]; n + 1]; // rce からほかの rce または T にたどり着けるもの
    let mut rc = vec![];
    for &(r, c, _) in rce {
        rc.push((r, c));
    }

    let Some(s) = rc.iter().position(|&(r, c)| r == dr && c == sc) else { return false; };
    let t = rc
        .iter()
        .position(|&(r, c)| r == tr && c == tc)
        .unwrap_or(n);
    if t == n {
        rc.push((tr, tc));
    }

    for (i, &(r, c, e)) in rce.iter().enumerate() {
        let v = g(a, (r, c), &rc);
        for (j, _) in rc.iter().enumerate() {
            if j != i && v[j] <= e {
                graph[i].push(j);
            }
        }
    }

    let mut visited = HashSet::new();
    let mut stack = vec![s];

    while let Some(i) = stack.pop() {
        for &j in &graph[i] {
            if j == t {
                return true;
            }
            if visited.insert(j) {
                stack.push(j);
            }
        }
    }

    false
}

fn main() {
    input! {
        (h, _): (usize, usize),
        a: [Chars; h],
        n: usize,
        rce: [(Usize1, Usize1, usize); n],
    }
    let yes = f(&a, &rce);
    println!("{}", if yes { "Yes" } else { "No" });
}
