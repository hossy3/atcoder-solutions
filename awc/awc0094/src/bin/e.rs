use std::collections::HashSet;

use proconio::{input, marker::Chars};

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

/// 2次元マップ上の sxy から gxy への最短経路を求める
fn shortest_2dmap(m: &mut [Vec<char>]) -> usize {
    use std::{cmp::Reverse, collections::BinaryHeap};

    let sxy = find_pos(m, 'S').pop().unwrap();

    let mut queue = BinaryHeap::new();
    m[sxy.0][sxy.1] = 'X';
    queue.push((Reverse(0), sxy));

    while let Some((Reverse(step), xy)) = queue.pop() {
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
            match m[xy.0][xy.1] {
                'G' => return step + 1,
                'O' => {
                    m[xy.0][xy.1] = 'X';
                    queue.push((Reverse(step + 1), xy))
                }
                'V' => {
                    let mut set = HashSet::new();
                    set.insert(xy);
                    let mut stack = vec![xy];
                    while let Some(xy) = stack.pop() {
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
                            if set.contains(&xy) {
                                continue;
                            }
                            if m[xy.0][xy.1] == 'V' {
                                stack.push(xy);
                                set.insert(xy);
                            }
                        }
                    }
                    for (x, y) in set {
                        m[x][y] = 'X';
                        queue.push((Reverse(step + 2), (x, y)))
                    }
                }
                'X' => {}
                _ => {
                    unreachable!()
                }
            }
        }
    }

    usize::MAX
}

fn main() {
    input! {
        h: usize,
        _w: usize,
        mut c: [Chars; h],
    }

    let result = shortest_2dmap(&mut c);
    if result == usize::MAX {
        println!("NO");
    } else {
        println!("{result}");
    }
}
