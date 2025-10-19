use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Chars};

fn f(s: &[Vec<char>]) -> (usize, usize) {
    let h = s.len();
    let w = s[0].len();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'T' {
                return (i, j);
            }
        }
    }
    unreachable!()
}

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let rest: usize = s
        .iter()
        .map(|s| s.iter().filter(|&&c| c == '#').count())
        .sum();
    let (ti, tj) = f(&s);
    let tuple = ((0isize, 0isize), (0, 0), (h, w), rest, 0); // ゴミ左上の位置、有効左上、有向右下

    let mut queue = VecDeque::new();
    queue.push_back(tuple); // (i0, j0)..(i1, j1)

    let mut set = HashSet::new();

    while let Some(((i0, j0), (i1, j1), (i2, j2), rest, count)) = queue.pop_front() {
        let count = count + 1;

        // ゴミを1つ上に移動する
        {
            let i0 = i0 - 1;
            let i3 = ti.wrapping_add_signed(-i0);
            let j3 = tj.wrapping_add_signed(-j0);
            if !(i1 <= i3 && i3 < i2 && j1 <= j3 && j3 < j2 && s[i3][j3] == '#') {
                if i0 + (i1 as isize) < 0 {
                    let mut rest = rest;
                    for j in j1..j2 {
                        if s[i1][j] == '#' {
                            rest -= 1;
                        }
                    }
                    if rest == 0 {
                        println!("{count}");
                        return;
                    }
                    let i1 = i1 + 1;
                    if set.insert(((i0, j0), (i1, j1), (i2, j2))) {
                        queue.push_back(((i0, j0), (i1, j1), (i2, j2), rest, count));
                    }
                } else {
                    if set.insert(((i0, j0), (i1, j1), (i2, j2))) {
                        queue.push_back(((i0, j0), (i1, j1), (i2, j2), rest, count));
                    }
                };
            }
        }

        // ゴミを1つ下に移動する
        {
            let i0 = i0 + 1;
            let i3 = ti.wrapping_add_signed(-i0);
            let j3 = tj.wrapping_add_signed(-j0);
            if !(i1 <= i3 && i3 < i2 && j1 <= j3 && j3 < j2 && s[i3][j3] == '#') {
                if i0 + (i2 as isize) > h as isize {
                    let mut rest = rest;
                    for j in j1..j2 {
                        if s[i2 - 1][j] == '#' {
                            rest -= 1;
                        }
                    }
                    if rest == 0 {
                        println!("{count}");
                        return;
                    }
                    let i2 = i2 - 1;
                    if set.insert(((i0, j0), (i1, j1), (i2, j2))) {
                        queue.push_back(((i0, j0), (i1, j1), (i2, j2), rest, count));
                    }
                } else {
                    if set.insert(((i0, j0), (i1, j1), (i2, j2))) {
                        queue.push_back(((i0, j0), (i1, j1), (i2, j2), rest, count));
                    }
                };
            }
        }

        // ゴミを1つ左に移動する
        {
            let j0 = j0 - 1;
            let i3 = ti.wrapping_add_signed(-i0);
            let j3 = tj.wrapping_add_signed(-j0);
            if !(i1 <= i3 && i3 < i2 && j1 <= j3 && j3 < j2 && s[i3][j3] == '#') {
                if j0 + (j1 as isize) < 0 {
                    let mut rest = rest;
                    for i in i1..i2 {
                        if s[i][j1] == '#' {
                            rest -= 1;
                        }
                    }
                    if rest == 0 {
                        println!("{count}");
                        return;
                    }
                    let j1 = j1 + 1;
                    if set.insert(((i0, j0), (i1, j1), (i2, j2))) {
                        queue.push_back(((i0, j0), (i1, j1), (i2, j2), rest, count));
                    }
                } else {
                    if set.insert(((i0, j0), (i1, j1), (i2, j2))) {
                        queue.push_back(((i0, j0), (i1, j1), (i2, j2), rest, count));
                    }
                };
            }
        }

        // ゴミを1つ右に移動する
        {
            let j0 = j0 + 1;
            let i3 = ti.wrapping_add_signed(-i0);
            let j3 = tj.wrapping_add_signed(-j0);
            if !(i1 <= i3 && i3 < i2 && j1 <= j3 && j3 < j2 && s[i3][j3] == '#') {
                if j0 + (j2 as isize) > w as isize {
                    let mut rest = rest;
                    for i in i1..i2 {
                        if s[i][j2 - 1] == '#' {
                            rest -= 1;
                        }
                    }
                    if rest == 0 {
                        println!("{count}");
                        return;
                    }
                    let j2 = j2 - 1;
                    if set.insert(((i0, j0), (i1, j1), (i2, j2))) {
                        queue.push_back(((i0, j0), (i1, j1), (i2, j2), rest, count));
                    }
                } else {
                    if set.insert(((i0, j0), (i1, j1), (i2, j2))) {
                        queue.push_back(((i0, j0), (i1, j1), (i2, j2), rest, count));
                    }
                };
            }
        }
    }

    println!("-1");
}
