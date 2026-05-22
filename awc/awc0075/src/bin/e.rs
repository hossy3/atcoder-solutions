use itertools::Itertools;
use proconio::{input, marker::Chars};

/// 2次元マップ上の sxy から gxy への最短経路を求める
fn shortest_2dmap(m: &[Vec<char>], sxy: &(usize, usize), gxy: &(usize, usize)) -> usize {
    use std::{cmp::Reverse, collections::BinaryHeap};

    let mut queue = BinaryHeap::new();
    let mut map = vec![vec![true; m[0].len()]; m.len()];
    map[sxy.0][sxy.1] = false;
    queue.push((Reverse(0), *sxy));

    while let Some((Reverse(step), xy)) = queue.pop() {
        if xy.0 == gxy.0 && xy.1 == gxy.1 {
            return step;
        }
        let a = [
            (xy.0.wrapping_sub(1), xy.1),
            (xy.0 + 1, xy.1),
            (xy.0, xy.1.wrapping_sub(1)),
            (xy.0, xy.1 + 1),
        ];
        for &(x, y) in &a {
            if x >= m.len() || y >= m[0].len() {
                continue;
            }
            if m[x][y] == '#' {
                continue;
            }
            if map[x][y] {
                map[x][y] = false;
                queue.push((Reverse(step + 1), (x, y)));
            }
        }
    }

    usize::MAX
}

fn find_pos(s: &[Vec<char>], c: char) -> Vec<(usize, usize)> {
    let mut result = vec![];
    for (i, row) in s.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == c {
                result.push((i, j));
            }
        }
    }
    result
}

fn main() {
    input! {
        h: usize,
        _w: usize,
        k: usize,
        s: [Chars; h],
    }

    let a = find_pos(&s, '@')[0];
    let g = find_pos(&s, 'G')[0];
    let kittens = find_pos(&s, 'F');

    let mut va = vec![usize::MAX; k];
    for (i, &xy) in kittens.iter().enumerate() {
        va[i] = shortest_2dmap(&s, &a, &xy);
    }

    let mut vg = vec![usize::MAX; k];
    for (i, &xy) in kittens.iter().enumerate() {
        vg[i] = shortest_2dmap(&s, &xy, &g);
    }

    let mut vk = vec![vec![usize::MAX; k]; k];
    for (i0, &xy0) in kittens.iter().enumerate() {
        for (i1, &xy1) in kittens.iter().enumerate() {
            vk[i0][i1] = shortest_2dmap(&s, &xy0, &xy1);
        }
    }

    let mut result = usize::MAX;
    for v in (0..k).permutations(k) {
        let mut cur = va[v[0]];
        for v in v.windows(2) {
            cur = cur.saturating_add(vk[v[0]][v[1]]);
        }
        cur = cur.saturating_add(vg[v[k - 1]]);
        result = result.min(cur);
    }
    if result == usize::MAX {
        println!("-1");
    } else {
        println!("{result}");
    }
}
