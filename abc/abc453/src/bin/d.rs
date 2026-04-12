use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn find_pos(m: &[Vec<char>], c: char) -> (usize, usize) {
    for (i, m) in m.iter().enumerate() {
        for (j, &c0) in m.iter().enumerate() {
            if c0 == c {
                return (i, j);
            }
        }
    }
    unreachable!()
}

fn shortest_2dmap(m: &[Vec<char>]) -> Vec<char> {
    const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    const DIR_CHARS: [char; 4] = ['U', 'D', 'L', 'R'];

    let sxy = find_pos(m, 'S');
    let gxy = find_pos(m, 'G');

    let h = m.len();
    let w = m[0].len();
    let mut parents = vec![vec![[usize::MAX; 4]; w]; h]; // parent_dir
    let mut queue = VecDeque::new();
    for (dir0, &(dx, dy)) in DIRS.iter().enumerate() {
        let x = sxy.0.wrapping_add_signed(dx);
        let y = sxy.1.wrapping_add_signed(dy);
        if x >= h || y >= w {
            continue;
        }
        queue.push_back(((x, y), dir0)); // (xy, dir)
    }

    while let Some((mut xy, mut dir)) = queue.pop_front() {
        if xy.0 == gxy.0 && xy.1 == gxy.1 {
            let mut results = vec![];
            while xy != sxy {
                results.push(DIR_CHARS[dir]);
                let parent_dir = parents[xy.0 as usize][xy.1 as usize][dir];
                let dir0 = if dir < 2 { 1 - dir } else { 5 - dir }; // 親に向かう方向
                xy = (
                    xy.0.wrapping_add_signed(DIRS[dir0].0),
                    xy.1.wrapping_add_signed(DIRS[dir0].1),
                );
                dir = parent_dir;
            }
            return results.into_iter().rev().collect();
        }

        for (dir0, &(dx, dy)) in DIRS.iter().enumerate() {
            match m[xy.0][xy.1] {
                '#' | 'S' => continue,
                '.' | 'G' => {}
                'o' => {
                    if dir0 != dir {
                        continue;
                    }
                }
                'x' => {
                    if dir0 == dir {
                        continue;
                    }
                }
                _ => unreachable!(),
            }

            let x = xy.0.wrapping_add_signed(dx);
            let y = xy.1.wrapping_add_signed(dy);
            if x >= h || y >= w {
                continue;
            }
            if parents[x][y][dir0] != usize::MAX {
                continue; // すでに進路がある
            }
            parents[x][y][dir0] = dir;
            queue.push_back(((x, y), dir0));
        }
    }

    vec![]
}

fn main() {
    input! {
        h: usize,
        _w: usize,
        s: [Chars; h],
    }

    let result = shortest_2dmap(&s);
    if result.len() > 0 {
        println!("Yes");
        println!("{}", result.iter().join(""));
    } else {
        println!("No");
    }
}
