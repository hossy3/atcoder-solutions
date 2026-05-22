use proconio::{input, marker::Chars};

/// 2次元マップ上の sxy から gxy への最短経路を求める
fn shortest_2dmap(
    m: &[Vec<char>],
    sxy: &(usize, usize),
    gxy: &(usize, usize),
    kittens: &[(usize, usize)],
) -> usize {
    use std::{cmp::Reverse, collections::BinaryHeap};

    let mut queue = BinaryHeap::new();
    let mut map = vec![vec![vec![true; 1 << kittens.len()]; m[0].len()]; m.len()];
    map[sxy.0][sxy.1][0] = false;
    queue.push((Reverse(0), *sxy, 0));

    while let Some((Reverse(step), xy, bits)) = queue.pop() {
        if xy.0 == gxy.0 && xy.1 == gxy.1 && bits == (1 << kittens.len()) - 1 {
            return step;
        }
        let a = [
            (xy.0.wrapping_sub(1), xy.1),
            (xy.0 + 1, xy.1),
            (xy.0, xy.1.wrapping_sub(1)),
            (xy.0, xy.1 + 1),
        ];
        for &(x, y) in &a {
            let mut bits = bits;
            if x >= m.len() || y >= m[0].len() {
                continue;
            }
            if m[x][y] == '#' {
                continue;
            }
            if let Some(i) = kittens.iter().position(|&xy0| xy0 == (x, y)) {
                bits |= 1 << i;
            }
            if map[x][y][bits] {
                map[x][y][bits] = false;
                queue.push((Reverse(step + 1), (x, y), bits));
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
        _k: usize,
        s: [Chars; h],
    }

    let a = find_pos(&s, '@')[0];
    let g = find_pos(&s, 'G')[0];
    let kittens = find_pos(&s, 'F');

    let result = shortest_2dmap(&s, &a, &g, &kittens);
    if result == usize::MAX {
        println!("-1");
    } else {
        println!("{result}");
    }
}
