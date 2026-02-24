use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn shortest_2dmap(m: &[Vec<char>], sxy: &(usize, usize), gxy: &(usize, usize)) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; m[0].len()]; m.len()];
    visited[sxy.0][sxy.1] = true;
    queue.push_back((0, *sxy));

    while let Some((step, xy)) = queue.pop_front() {
        if xy.0 == gxy.0 && xy.1 == gxy.1 {
            return step;
        }
        let a = [
            (xy.0.wrapping_sub(1), xy.1),
            (xy.0 + 1, xy.1),
            (xy.0, xy.1.wrapping_sub(1)),
            (xy.0, xy.1 + 1),
        ];
        for &xy in &a {
            if xy.0 >= m.len() || xy.1 >= m[0].len() {
                continue;
            }
            if !visited[xy.0][xy.1] {
                visited[xy.0][xy.1] = true;
                if m[xy.0][xy.1] == '.' {
                    queue.push_front((step, xy));
                } else {
                    queue.push_back((step + 1, xy));
                }
            }
        }
    }
    panic!();
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }
    let result = shortest_2dmap(&s, &(0, 0), &(n - 1, m - 1));
    println!("{result}");
}
