use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn f(s: &[Vec<char>]) -> usize {
    let h = s.len();
    let w = s[0].len();

    let mut queue = VecDeque::new();
    queue.push_back((0, (0, 0), 3)); // step, dir (UDLR), (i, j)

    let mut steps = vec![vec![[usize::MAX; 4]; w]; h];
    steps[0][0][3] = 0;

    while let Some((step, (i, j), dir)) = queue.pop_front() {
        if i == h - 1 && j == w {
            return step;
        }

        // そのまま移動する向き
        let dir0 = match s[i][j] {
            'A' => dir,
            'B' => (dir + 2) % 4,
            'C' => 3 - dir,
            _ => unreachable!(),
        };

        // 反対向き
        let dir1 = match dir {
            0 => 1,
            1 => 0,
            2 => 3,
            3 => 2,
            _ => unreachable!(),
        };

        for dir in 0..4 {
            if dir == dir1 {
                continue; // 置き換えても反対向きには進まない
            }
            let step = if dir == dir0 { step } else { step + 1 };
            match dir {
                0 => {
                    let i = i.wrapping_sub(1);
                    if i < h && steps[i][j][dir] > step {
                        steps[i][j][dir] = step;
                        if dir == dir0 {
                            queue.push_front((step, (i, j), dir));
                        } else {
                            queue.push_back((step, (i, j), dir));
                        }
                    }
                }
                1 => {
                    let i = i.wrapping_add(1);
                    if i < h && steps[i][j][dir] > step {
                        steps[i][j][dir] = step;
                        if dir == dir0 {
                            queue.push_front((step, (i, j), dir));
                        } else {
                            queue.push_back((step, (i, j), dir));
                        }
                    }
                }
                2 => {
                    let j = j.wrapping_sub(1);
                    if j < w && steps[i][j][dir] > step {
                        steps[i][j][dir] = step;
                        if dir == dir0 {
                            queue.push_front((step, (i, j), dir));
                        } else {
                            queue.push_back((step, (i, j), dir));
                        }
                    }
                }
                3 => {
                    let j = j.wrapping_add(1);
                    if j < w && steps[i][j][dir] > step {
                        steps[i][j][dir] = step;
                        if dir == dir0 {
                            queue.push_front((step, (i, j), dir));
                        } else {
                            queue.push_back((step, (i, j), dir));
                        }
                    } else if i == h - 1 && j == w {
                        if dir == dir0 {
                            queue.push_front((step, (i, j), dir));
                        } else {
                            queue.push_back((step, (i, j), dir));
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    usize::MAX
}

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            h: usize,
            _: usize,
            s: [Chars; h],
        }
        let result = f(&s);
        println!("{result}");
    }
}
