use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    time::Instant,
};

use itertools::Itertools;
use proconio::{input, marker::Chars};

#[derive(Clone, Copy, Debug)]
struct Neighbor {
    prev: usize,
    next: usize,
}

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const LIMIT_TIME: f64 = 1.9;

// 方向の列を返す (UDLR)
fn dijkstra(
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
    v: &[Vec<char>],
    h: &[Vec<char>],
    dir_indexes: &[usize],
) -> Vec<usize> {
    let n = v.len();
    let mut mat = vec![vec![usize::MAX; n]; n];
    let mut queue = VecDeque::new();
    queue.push_back((x0, y0, 0)); // 初手の方向は適当

    while let Some((x, y, dir)) = queue.pop_front() {
        if mat[x][y] != usize::MAX {
            continue;
        }
        mat[x][y] = dir;

        if (x, y) == (x1, y1) {
            break;
        }

        for &dir in dir_indexes {
            let (dx, dy) = DIRS[dir];
            let nx = x.wrapping_add_signed(dx);
            let ny = y.wrapping_add_signed(dy);
            if nx >= n || ny >= n {
                continue;
            }
            // if dx == -1 && h[nx].len() == 1 && y == 1 {
            //     eprintln!("{x} {y} {nx} {ny}");
            // }
            if (dx == -1 && h[nx][y] == '1')
                || (dx == 1 && h[x][y] == '1')
                || (dy == -1 && v[x][ny] == '1')
                || (dy == 1 && v[x][y] == '1')
            {
                continue;
            }
            queue.push_back((nx, ny, dir));
        }
    }

    let mut results = vec![];
    let mut x = x1;
    let mut y = y1;
    while (x != x0) || (y != y0) {
        let dir = mat[x][y];
        results.push(dir);
        let (dx, dy) = DIRS[dir];
        x = x.wrapping_add_signed(-dx);
        y = y.wrapping_add_signed(-dy);
    }
    results.reverse();
    results
}

// 解答をまとめる
fn f(
    n: usize,
    xy0: (usize, usize),
    dirs: &[usize],
    neighbors: &[Neighbor],
    c_max: usize,
) -> (
    usize,
    Vec<Vec<usize>>,
    Vec<(usize, usize, usize, usize, usize)>,
) {
    let mut s = vec![vec![usize::MAX; n]; n];
    let mut results = vec![(usize::MAX, usize::MAX); dirs.len()]; // results[q * c_max + c] = (q0 * c_max + c0, dir)
    let (mut x, mut y) = xy0;
    let mut qc = 0; // 次に新規なら使う状態 q * c_max + c (既存の色・状態の組み合わせを使うこともある)
    let mut reuseable = [(usize::MAX, false); 4]; // UDLR に対応する、その状態で再利用可能な色番号、状態変更指定済みか
    let mut i_to_qc: Vec<usize> = vec![]; // dirs の各要素が results の何番目に対応するか
    let mut counts = vec![0; dirs.len()]; // counts[q * c_max + c] 出現回数
    let mut empty_qc = BTreeSet::new(); // 空き番号 qc
    let g = |c: usize, q: usize| q * c_max + c;
    let mut i_to_xy = vec![(usize::MAX, usize::MAX); dirs.len()];

    for (i, &dir) in dirs.iter().enumerate() {
        // そのマスを最後に通る場合は既存の組み合わせでも良い
        let mut qc0 = qc;
        if reuseable[dir].0 == usize::MAX {
            reuseable[dir].0 = qc0;
            if neighbors[i].next != usize::MAX {
                reuseable[dir].1 = true;
            }
        } else {
            if neighbors[i].next == usize::MAX {
                (qc0, _) = reuseable[dir];
            } else if !reuseable[dir].1 {
                (qc0, _) = reuseable[dir];
                reuseable[dir].1 = true;
            }
        }

        let c0 = qc0 % c_max;
        if s[x][y] == usize::MAX {
            s[x][y] = c0;
        }
        i_to_xy[i] = (x, y);
        if neighbors[i].prev != usize::MAX {
            // eprintln!("{} {} {:?}", i, neighbors[i].prev, i_to_index[neighbors[i].prev]);
            results[i_to_qc[neighbors[i].prev]].0 =
                (results[i_to_qc[neighbors[i].prev]].0 / c_max) * c_max + c0;
        }

        if qc0 == qc {
            let qc1 = qc; // 現在の値を覚えておく
            qc += 1;
            if qc % c_max == 0 {
                reuseable = [(usize::MAX, false); 4];
            }
            results[qc1] = (qc, dir); // 再利用しないときに追加
        }
        i_to_qc.push(qc0);
        counts[qc0] += 1;

        let (dx, dy) = DIRS[dir];
        x = x.wrapping_add_signed(dx);
        y = y.wrapping_add_signed(dy);
    }

    for x in 0..n {
        for y in 0..n {
            if s[x][y] == usize::MAX {
                s[x][y] = 0; // 通らないのでなんでも良い
            }
        }
    }

    // 最大の状態数を使わない場合は次の状態数を小さくする
    let q_max = (results.len() - 1) / c_max;
    for i in 0..results.len() {
        let q = (results[i].0 / c_max).min(q_max);
        let c = results[i].0 % c_max;
        results[i].0 = q * c_max + c;
    }

    // results を切りの良い数まで増やす
    while results.len() % c_max > 0 {
        let qc = results.len();
        results.push((usize::MAX, usize::MAX));
        counts.push(0);
        empty_qc.insert(qc);
    }

    let mut qc_to_i: Vec<usize> = vec![usize::MAX; results.len()];
    for (i, &qc) in i_to_qc.iter().enumerate() {
        if counts[qc] == 1 {
            qc_to_i[qc] = i;
        }
    }

    // 状態をまとめられる場合はまとめる
    for q in (0..=q_max).rev() {
        let mut map = HashMap::new();
        for c in 0..c_max {
            let qc = g(c, q);
            if counts[qc] != 1 {
                continue;
            }
            let (qc0, dir) = results[qc];
            if dir == usize::MAX {
                continue;
            }
            let q0 = qc0 / c_max;
            let c0 = qc0 % c_max;
            if q0 != q {
                continue;
            }
            let i = qc_to_i[qc];
            let i1 = neighbors[i].next;
            if i1 == usize::MAX {
                continue; // 最後にそのマスに着く場合。すでに考慮しているのでスキップできる
            }
            map.entry((c0, results[i_to_qc[i1]], dir))
                .or_insert(vec![])
                .push(i);
        }

        for (_, v) in map {
            let i2 = v[0];
            let qc2 = i_to_qc[i2];
            for &i in &v[1..] {
                let i3 = neighbors[i].prev;
                if i3 < usize::MAX {
                    let qc3 = i_to_qc[i3];
                    if counts[qc3] != 1 {
                        continue;
                    }
                    results[qc3].0 = g(qc2 % c_max, results[qc3].0 / c_max);
                } else {
                    let (x, y) = i_to_xy[i];
                    s[x][y] = qc2 % c_max;
                }

                let qc = i_to_qc[i];
                counts[qc] -= 1;
                counts[qc2] += 1;
                i_to_qc[i] = qc2;
                qc_to_i[qc] = usize::MAX; // 該当しなくなるので対応無しとする
                qc_to_i[qc2] = usize::MAX; // 複数該当するので対応無しとする
                empty_qc.insert(qc);
            }
        }
    }

    // まとめたあとの隙間を埋める
    for qc in (1..results.len()).rev() {
        if empty_qc.len() < c_max {
            break; // これ以上圧縮しても意味がない
        }
        // if empty_qc
        //     .range(((qc / c_max) * c_max)..((qc / c_max) * (c_max + 1)))
        //     .next()
        //     .is_none()
        // {
        //     continue; // その列は埋まっているので操作する意味がない
        // }
        if empty_qc.contains(&qc) {
            continue;
        }
        let Some(&qc0) = empty_qc.last() else {
            unreachable!()
        };
        if qc >= qc0 {
            continue; // 手前の空きを埋めることはしない
        }
        let i = qc_to_i[qc];
        if i == 0 || i >= dirs.len() - 1 {
            continue;
        }
        let i1 = i - 1; // 1手前
        let qc1 = i_to_qc[i1];
        if counts[qc1] != 1 {
            continue;
        }
        let i2 = i + 1; // 1手先
        let qc2 = i_to_qc[i2];

        let i3 = neighbors[i].prev; // 同じマスに前回通る (色をそこで設定する)
        if i3 < usize::MAX {
            let qc3 = i_to_qc[i3];
            if counts[qc3] != 1 {
                continue;
            }
            results[qc3].0 = g(qc0 % c_max, results[qc3].0 / c_max);
        } else {
            // そのマス初回
            let (x, y) = i_to_xy[i];
            s[x][y] = qc0 % c_max;
        }
        results[qc1].0 = g(results[qc1].0 % c_max, qc0 / c_max);
        results[qc0] = (g(results[qc].0 % c_max, qc2 / c_max), results[qc].1);
        results[qc] = (usize::MAX, usize::MAX);
        counts[qc0] += 1;
        counts[qc] -= 1;
        i_to_qc[i] = qc0;
        qc_to_i[qc0] = i;
        qc_to_i[qc] = usize::MAX;
        empty_qc.remove(&qc0);
        empty_qc.insert(qc);

        // もし1行すべて空になるなら、空候補から除く
        let Some(&qc) = empty_qc.last() else {
            unreachable!();
        };
        if qc % c_max == c_max - 1 {
            if empty_qc
                .iter()
                .rev()
                .take(c_max - 1)
                .enumerate()
                .all(|(i, qc0)| i + qc0 == qc)
            {
                for _ in 0..c_max {
                    empty_qc.pop_last();
                }
            }
        }
    }

    let mut q_set = BTreeSet::new();
    for (qc, &(_, dir)) in results.iter().enumerate() {
        if dir != usize::MAX {
            q_set.insert(qc / c_max);
        }
    }
    let mut vq = vec![usize::MAX; results.len() / c_max];
    for (i, &q) in q_set.iter().enumerate() {
        vq[q] = i;
    }

    let results: Vec<_> = results
        .iter()
        .enumerate()
        .filter(|&(_, &(_, dir))| dir != usize::MAX)
        .map(|(qc1, &(qc, dir))| {
            (
                qc1 % c_max,
                vq[qc1 / c_max],
                qc % c_max,
                vq[qc / c_max],
                dir,
            )
        })
        .collect();

    (q_set.len(), s, results)
}

#[allow(dead_code)]
fn validate(
    c_max: usize,
    s: &[Vec<usize>],
    results: &[(usize, usize)],
    (mut x, mut y): (usize, usize),
    dirs: &[usize],
) {
    let g = |c: usize, q: usize| q * c_max + c;

    let mut vv = vec![];

    let mut q = 0;
    let mut s: Vec<_> = s.iter().map(|s| s.clone()).collect();
    for (_, &dir) in dirs.iter().enumerate() {
        let c = s[x][y];
        let (qc0, dir0) = results[g(c, q)];
        vv.push((x, y, c, q, qc0, dir0));
        s[x][y] = qc0 % c_max;
        q = qc0 / c_max;

        let (dx, dy) = DIRS[dir];
        x = x.wrapping_add_signed(dx);
        y = y.wrapping_add_signed(dy);
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        _t: usize,
        v: [Chars; n],
        h: [Chars; n - 1],
        xy: [(usize, usize); k],
    }

    let mut c = usize::MAX;
    let mut q = usize::MAX;
    let mut s = vec![];
    let mut results = vec![];
    let timer = Instant::now();

    for &dir_indexes in &[[0, 1, 2, 3]] {
    // for dir_indexes in [0, 1, 2, 3].iter().cloned().permutations(4) {
        // 一次元の通過列と、それぞれのマスにいつ到達するかを得る
        let mut dirs = vec![];
        let mut mat = vec![vec![vec![]; n]; n];

        for ((x0, y0), (x1, y1)) in xy.windows(2).map(|w| (w[0], w[1])) {
            let dirs0 = dijkstra(x0, y0, x1, y1, &v, &h, &dir_indexes);
            let mut cx = x0;
            let mut cy = y0;
            for dir in dirs0 {
                mat[cx][cy].push(dirs.len());
                dirs.push(dir);
                let (dx, dy) = DIRS[dir];
                cx = cx.wrapping_add_signed(dx);
                cy = cy.wrapping_add_signed(dy);
            }
        }

        // 同じマスに前後に何手目に来るか
        let mut neighbors = vec![
            Neighbor {
                prev: usize::MAX,
                next: usize::MAX
            };
            dirs.len()
        ];
        for x in 0..n {
            for y in 0..n {
                for (i, j) in mat[x][y].windows(2).map(|v| (v[0], v[1])) {
                    neighbors[i].next = j;
                    neighbors[j].prev = i;
                }
            }
        }

        // 解答をまとめる
        let x = (dirs.len() as f64).sqrt().ceil() as usize;
        let c_max = if (x - 1) * (x - 1) >= dirs.len() {
            x - 1
        } else {
            x
        };

        // for c0 in 31..=31 {
        for c0 in (c_max / 2)..=c_max {
            let (q0, s0, results0) = f(n, xy[0], &dirs, &neighbors, c0);
            if c0 + q0 < c.saturating_add(q) {
                c = c0;
                q = q0;
                s = s0;
                results = results0;
            }
            if timer.elapsed().as_secs_f64() >= LIMIT_TIME {
                break;
            }
        }
        if timer.elapsed().as_secs_f64() >= LIMIT_TIME {
            break;
        }
    }

    let m = results.len();
    println!("{c} {q} {m}");
    for s in s {
        println!("{}", s.iter().join(" "));
    }
    for (c, q, a, s, dir) in results {
        let d = ['U', 'D', 'L', 'R'][dir];
        println!("{c} {q} {a} {s} {d}");
    }
}
