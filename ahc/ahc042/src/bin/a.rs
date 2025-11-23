use proconio::{input, marker::Chars};

fn pos(n: usize, i: usize, j: usize) -> usize {
    i * n + j
}

fn moved(n: usize, v: &Vec<char>, dir: char, i: usize) -> Vec<char> {
    let mut v = v.clone();
    match dir {
        'L' => {
            for j in 0..(n - 1) {
                v[pos(n, i, j)] = v[pos(n, i, j + 1)];
            }
            v[pos(n, i, n - 1)] = '.';
        }
        'R' => {
            for j in (0..(n - 1)).rev() {
                v[pos(n, i, j + 1)] = v[pos(n, i, j)];
            }
            v[pos(n, i, 0)] = '.';
        }
        'U' => {
            for j in 0..(n - 1) {
                v[pos(n, j, i)] = v[pos(n, j + 1, i)];
            }
            v[pos(n, n - 1, i)] = '.';
        }
        'D' => {
            for j in (0..(n - 1)).rev() {
                v[pos(n, j + 1, i)] = v[pos(n, j, i)];
            }
            v[pos(n, 0, i)] = '.';
        }
        _ => unreachable!(),
    }
    v
}

// 同じ操作だけを繰り返して鬼を外に出せるなら、その最短方法を返す。長さ 0 は存在しない場合
fn f0(n: usize, v: &Vec<char>) -> Vec<(char, usize)> {
    let dirs = ['L', 'R', 'U', 'D'];
    let mut candidates = vec![];

    for i in 0..n {
        for &dir in &dirs {
            match dir {
                'L' => {
                    let Some(j) = (0..n).position(|j| v[pos(n, i, j)] == 'x') else {
                        continue;
                    };
                    if (0..j).any(|j| v[pos(n, i, j)] == 'o') {
                        continue;
                    }
                    if candidates.len() > 0 && candidates.len() <= j + 1 {
                        continue;
                    }
                    candidates = vec![(dir, i); j + 1];
                }
                'R' => {
                    let Some(j) = (0..n).position(|j| v[pos(n, i, j)] == 'x') else {
                        continue;
                    };
                    if (j..n).any(|j| v[pos(n, i, j)] == 'o') {
                        continue;
                    }
                    if candidates.len() > 0 && candidates.len() <= j + 1 {
                        continue;
                    }
                    candidates = vec![(dir, i); n - j];
                }
                'U' => {
                    let Some(j) = (0..n).position(|j| v[pos(n, j, i)] == 'x') else {
                        continue;
                    };
                    if (0..j).any(|j| v[pos(n, j, i)] == 'o') {
                        continue;
                    }
                    if candidates.len() > 0 && candidates.len() <= j + 1 {
                        continue;
                    }
                    candidates = vec![(dir, i); j + 1];
                }
                'D' => {
                    let Some(j) = (0..n).position(|j| v[pos(n, j, i)] == 'x') else {
                        continue;
                    };
                    if (j..n).any(|j| v[pos(n, j, i)] == 'o') {
                        continue;
                    }
                    if candidates.len() > 0 && candidates.len() <= j + 1 {
                        continue;
                    }
                    candidates = vec![(dir, i); n - j];
                }
                _ => unreachable!(),
            }
        }
    }

    candidates
}

fn f(n: usize, v: &Vec<char>) -> Vec<(char, usize)> {
    let dirs = ['L', 'R', 'U', 'D'];
    let mut candidates = f0(n, v);

    // 1回折り曲げてやり直す
    for i in 0..n {
        for &dir in &dirs {
            let mut v = v.clone();
            for k in 0..n {
                match dir {
                    'L' => {
                        if v[pos(n, i, 0)] != '.' {
                            break;
                        }
                        v = moved(n, &v, dir, i);
                        let mut cand0 = f0(n, &v);
                        if cand0.len() == 0 {
                            continue;
                        }
                        if candidates.len() > 0 && candidates.len() <= cand0.len() + k + 1 {
                            continue;
                        }
                        candidates = vec![(dir, i); k + 1];
                        candidates.append(&mut cand0);
                    }
                    'R' => {
                        if v[pos(n, i, n - 1)] != '.' {
                            break;
                        }
                        v = moved(n, &v, dir, i);
                        let mut cand0 = f0(n, &v);
                        if cand0.len() == 0 {
                            continue;
                        }
                        if candidates.len() > 0 && candidates.len() <= cand0.len() + k + 1 {
                            continue;
                        }
                        candidates = vec![(dir, i); k + 1];
                        candidates.append(&mut cand0);
                    }
                    'U' => {
                        if v[pos(n, 0, i)] != '.' {
                            break;
                        }
                        v = moved(n, &v, dir, i);
                        let mut cand0 = f0(n, &v);
                        if cand0.len() == 0 {
                            continue;
                        }
                        if candidates.len() > 0 && candidates.len() <= cand0.len() + k + 1 {
                            continue;
                        }
                        candidates = vec![(dir, i); k + 1];
                        candidates.append(&mut cand0);
                    }
                    'D' => {
                        if v[pos(n, n - 1, i)] != '.' {
                            break;
                        }
                        v = moved(n, &v, dir, i);
                        let mut cand0 = f0(n, &v);
                        if cand0.len() == 0 {
                            continue;
                        }
                        if candidates.len() > 0 && candidates.len() <= cand0.len() + k + 1 {
                            continue;
                        }
                        candidates = vec![(dir, i); k + 1];
                        candidates.append(&mut cand0);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    candidates
}

fn main() {
    input! {
        n: usize,
        c: [Chars; n],
    }

    let mut v = vec![];
    for c in &c {
        v.append(&mut c.clone());
    }

    for _ in 0..(2 * n) {
        let results = f(n, &v);
        if results.len() == 0 {
            break;
        }
        for (dir, i) in results {
            println!("{dir} {i}");
            v = moved(n, &v, dir, i);
        }
    }
}
