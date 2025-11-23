use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn is_walkable(c: char) -> bool {
    match c {
        '#' | '>' | 'v' | '<' | '^' => false,
        _ => true,
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }

    let mut m = vec![vec![true; w + 2]; h + 2]; // true: 通れる
    for i in 0..(h + 2) {
        m[i][0] = false;
        m[i][w + 1] = false;
    }
    for j in 0..(w + 2) {
        m[0][j] = false;
        m[h + 1][j] = false;
    }

    let mut start = (0, 0);
    let mut goal = (0, 0);

    for i in 0..h {
        for j in 0..w {
            match a[i][j] {
                '.' => {}
                '#' => {
                    m[i + 1][j + 1] = false;
                }
                '>' => {
                    m[i + 1][j + 1] = false;
                    for j0 in (j + 1)..w {
                        if is_walkable(a[i][j0]) {
                            m[i + 1][j0 + 1] = false;
                        } else {
                            break;
                        }
                    }
                    m[i + 1][j + 2] = false;
                }
                'v' => {
                    m[i + 1][j + 1] = false;
                    for i0 in (i + 1)..h {
                        if is_walkable(a[i0][j]) {
                            m[i0 + 1][j + 1] = false;
                        } else {
                            break;
                        }
                    }
                }
                '<' => {
                    m[i + 1][j + 1] = false;
                    for j0 in (0..j).rev() {
                        if is_walkable(a[i][j0]) {
                            m[i + 1][j0 + 1] = false;
                        } else {
                            break;
                        }
                    }
                }
                '^' => {
                    m[i + 1][j + 1] = false;
                    for i0 in (0..i).rev() {
                        if is_walkable(a[i0][j]) {
                            m[i0 + 1][j + 1] = false;
                        } else {
                            break;
                        }
                    }
                }
                'S' => {
                    start = (i + 1, j + 1);
                }
                'G' => {
                    goal = (i + 1, j + 1);
                }
                _ => panic!(),
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    m[start.0][start.1] = false;

    while let Some(((i, j), step)) = queue.pop_front() {
        if (i, j) == goal {
            println!("{step}");
            return;
        }
        if m[i][j + 1] {
            queue.push_back(((i, j + 1), step + 1));
            m[i][j + 1] = false;
        }
        if m[i + 1][j] {
            queue.push_back(((i + 1, j), step + 1));
            m[i + 1][j] = false;
        }
        if m[i][j - 1] {
            queue.push_back(((i, j - 1), step + 1));
            m[i][j - 1] = false;
        }
        if m[i - 1][j] {
            queue.push_back(((i - 1, j), step + 1));
            m[i - 1][j] = false;
        }
    }

    println!("-1");
}
