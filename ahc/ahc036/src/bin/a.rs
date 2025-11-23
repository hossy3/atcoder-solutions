use std::{
    collections::{BTreeMap, BTreeSet},
    mem::swap,
};

use itertools::Itertools;
use proconio::input;

trait BitTest {
    fn bit_test(&self, i: usize) -> bool;
}

impl BitTest for usize {
    fn bit_test(&self, i: usize) -> bool {
        self & (1 << i) != 0
    }
}

fn hilbert_order(mut x: usize, mut y: usize) -> usize {
    const MAX: usize = 1 << 10;
    let mut d = 0;
    let mut s = MAX >> 1;
    while s > 0 {
        let rx = (x & s) > 0;
        let ry = (y & s) > 0;
        let r = (if rx { 3 } else { 0 }) ^ (if ry { 1 } else { 0 });
        d += s * s * r;
        s = s >> 1;
        if ry {
            continue;
        }
        if rx {
            x = MAX - 1 - x;
            y = MAX - 1 - y;
        }
        swap(&mut x, &mut y);
    }
    d
}

fn build_a(la: usize, xy: &[(usize, usize)], graph: &[Vec<usize>]) -> Vec<usize> {
    let mut v = vec![];
    v.reserve(xy.len());
    for (i, &(x, y)) in xy.iter().enumerate() {
        v.push((hilbert_order(x, y), i));
    }
    v.sort();

    let mut a = vec![0; la];
    for (i, &(_, j)) in v.iter().enumerate() {
        a[i] = j;
    }

    // 少し順番を入れ替える
    let n = graph.len();
    for diff in [1, 2, 1, 3, 1, 2, 1, 4, 1, 2, 1, 3, 1, 2, 1] {
        for i in 0..(n - diff) {
            let mut score0 = 0; // 入れ替え前
            if i >= diff && graph[a[i]].contains(&a[i - diff]) {
                score0 += 1;
            }
            if i < n - diff - 1 && graph[a[i + diff]].contains(&a[i + diff + 1]) {
                score0 += 1;
            }

            let mut score1 = 0; // 入れ替え後
            if i >= diff && i < n - diff && graph[a[i + diff]].contains(&a[i - diff]) {
                score1 += 1;
            }
            if i < n - diff - 1 && graph[a[i]].contains(&a[i + diff + 1]) {
                score1 += 1;
            }

            if score0 < score1 {
                a.swap(i, i + diff);
            }
        }
    }

    // 後ろの方に環状線のような経路を、可能な分だけ追加する
    let mut i = n;
    if i < la {
        let path = build_a1(xy, graph);
        let m = path.len().min(la - i);
        a[i..(i + m)].copy_from_slice(&path[..m]);
        i += m;
    }
    if i < la {
        let path = build_a2(xy, graph);
        let m = path.len().min(la - i);
        a[i..(i + m)].copy_from_slice(&path[..m]);
        i += m;
    }
    if i < la {
        let path = build_a3(xy, graph);
        let m = path.len().min(la - i);
        a[i..(i + m)].copy_from_slice(&path[..m]);
    }

    for (i, &u) in a.iter().enumerate() {
        let x0 = ((xy[u].0 + 12) as f64 + 0.5) / 1024.0;
        let y0 = ((1024 - xy[u].1) as f64 - 0.5) / 1024.0;
        eprintln!("{i}: [{x0:.4}, {y0:.4}]");
    }

    a
}

// 右方向に隣接関係のあるノードをつないだパスを 1本だけ返す
fn build_a1(xy: &[(usize, usize)], graph: &[Vec<usize>]) -> Vec<usize> {
    // ヒルベルトが苦手な線からの距離。小さいほど良い
    let score = |i: usize| 1024usize.abs_diff(xy[i].1 * 2 + 1);

    let Some((_, _, mut cur)) = xy
        .iter()
        .enumerate()
        .map(|(i, &(x, y))| (x, y, i))
        .sorted()
        .next() else { unreachable!() };

    let mut results = Vec::new();
    results.push(cur);

    loop {
        let mut next = cur;
        for &i in &graph[cur] {
            if xy[i].0 > xy[cur].0 && (next == cur || score(i) < score(next)) {
                next = i;
            }
        }
        if next == cur {
            break;
        }
        cur = next;
        results.push(cur);
    }

    results
}

// 下方向に隣接関係のあるノードをつないだパスを 1本だけ返す
fn build_a2(xy: &[(usize, usize)], graph: &[Vec<usize>]) -> Vec<usize> {
    // ヒルベルトが苦手な線からの距離。小さいほど良い
    let score = |i: usize| 1024usize.abs_diff(xy[i].0 * 2 + 1);

    let Some((_, _, mut cur)) = xy
        .iter()
        .enumerate()
        .map(|(i, &(x, y))| (y, x, i))
        .sorted()
        .next() else { unreachable!() };

    let mut results = Vec::new();
    results.push(cur);

    loop {
        let mut next = cur;
        for &i in &graph[cur] {
            if xy[i].1 > xy[cur].1 && (next == cur || score(i) < score(next)) {
                next = i;
            }
        }
        if next == cur {
            break;
        }
        cur = next;
        results.push(cur);
    }

    results
}

// 外周かららせん状に隣接関係のあるノードをつないだパスを返す。途中途切れることがある
fn build_a3(xy: &[(usize, usize)], graph: &[Vec<usize>]) -> Vec<usize> {
    const MIDDLE: usize = 500;
    let mut results = Vec::new();
    let score = |(x, y): (usize, usize)| MIDDLE.abs_diff(x).max(MIDDLE.abs_diff(y));

    let xyi: Vec<_> = xy
        .iter()
        .enumerate()
        .map(|(i, &xy)| (score(xy), i))
        .sorted()
        .map(|(_, i)| i)
        .collect();
    let mut visited = vec![false; xy.len()];

    for mut cur in xyi {
        if visited[cur] {
            continue;
        }
        visited[cur] = true;
        results.push(cur);

        loop {
            let mut next = cur;
            for &i in &graph[cur] {
                if !visited[i] && (next == cur || score(xy[i]) > score(xy[next])) {
                    next = i;
                }
            }
            if next == cur {
                break;
            }
            cur = next;
            visited[cur] = true;
            results.push(cur);
        }
    }

    results
}

// 1手同士にできる場所 groups[u] = { v: a の位置, ... } を返す
fn build_groups(
    la: usize,
    lb: usize,
    graph: &[Vec<usize>],
    a: &[usize],
) -> Vec<BTreeMap<usize, usize>> {
    let n = graph.len();
    let mut groups = vec![BTreeMap::new(); n];

    for (i, &u) in a.iter().enumerate() {
        let i0 = (i + lb).min(la);

        let mut visited = vec![true; n];
        for &u in &a[i..i0] {
            visited[u] = false;
        }
        let mut group = BTreeSet::new();
        let mut stack = vec![u];
        while let Some(v) = stack.pop() {
            if visited[v] {
                continue;
            }
            visited[v] = true;
            group.insert(v);
            for &v in &graph[v] {
                if visited[v] {
                    continue;
                }
                stack.push(v);
            }
        }

        for &u in &group {
            for &v in &group {
                groups[u].insert(v, i);
                groups[v].insert(u, i);
            }
        }
    }

    groups
}

fn build_d(graph: &[Vec<usize>], groups: &[BTreeMap<usize, usize>]) -> Vec<Vec<usize>> {
    let n = groups.len();
    let mut d = vec![vec![n + 1; n]; n];
    for i in 0..n {
        d[i][i] = 0;
    }

    for (u, v) in graph.iter().enumerate() {
        for (&v, _) in &groups[u] {
            d[u][v] = d[u][v].min(1);
        }

        for &v in v {
            for (&v, _) in &groups[v] {
                d[u][v] = d[u][v].min(1);
            }
        }
    }

    floyd_warshall(d)
}

fn floyd_warshall(mut d: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = d.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[i][j] = d[i][j].min(d[i][k] + d[k][j]);
            }
        }
    }
    d
}

// cur から pos_to に辿るときに、cur を含む適当な a[i..(i + lb)] となる i を返す
fn calc_pos(
    groups: &[BTreeMap<usize, usize>],
    d: &[Vec<usize>],
    cur: usize,
    pos_to: usize,
) -> usize {
    let n = d.len();

    let mut i = 0;
    let mut cost = n + 1;
    for (&u0, &i0) in &groups[cur] {
        let cost0 = d[u0][pos_to];
        if cost > cost0 {
            i = i0;
            cost = cost0;
        }
    }

    i
}

// コスト0で動ける範囲で動く
fn walk0(
    cur: usize,
    pos_to: usize,
    graph: &[Vec<usize>],
    d: &[Vec<usize>],
    b: &[usize],
) -> Vec<usize> {
    let mut visited = vec![false; graph.len()];

    let mut stack = vec![(cur, vec![])];
    visited[cur] = true;

    let mut candidates = vec![];

    while let Some((cur, path)) = stack.pop() {
        if cur == pos_to {
            return path;
        }
        candidates.push((d[cur][pos_to], cur, path.clone()));

        for &p in &graph[cur] {
            if visited[p] || !b.contains(&p) {
                continue;
            }
            visited[p] = true;
            let mut path = path.clone();
            path.push(p);
            stack.push((p, path));
        }
    }

    candidates.sort();

    let (_, _, path) = candidates[0].clone();
    path
}

// コスト1で動けるうち、もっとも良いパスの、初手 (隣) を返す
fn walk1(
    cur: usize,
    pos_to: usize,
    graph: &[Vec<usize>],
    groups: &[BTreeMap<usize, usize>],
    d: &[Vec<usize>],
) -> usize {
    let mut candidate = (d[cur][pos_to], cur);
    for &u in &graph[cur] {
        if candidate.0 >= d[u][pos_to] {
            candidate = (d[u][pos_to], u);
        }
        for (&p, _) in &groups[u] {
            if candidate.0 >= d[p][pos_to] {
                candidate = (d[p][pos_to], u);
            }
        }
    }
    let (_, u) = candidate;
    u
}

fn main() {
    input! {
        (n, m, nt, la, lb): (usize, usize, usize, usize, usize),
        uv: [(usize, usize); m],
        t: [usize; nt],
        xy: [(usize, usize); n],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let a = build_a(la, &xy, &graph);
    println!("{}", a.iter().join(" "));

    let groups = build_groups(la, lb, &graph, &a);
    let d = build_d(&graph, &groups);

    let mut b = vec![usize::MAX; lb];
    let mut cur = 0;

    {
        // 次の中継点に向かいやすい b を得る
        let pos_to = t[0];
        let cur1 = walk1(cur, pos_to, &graph, &groups, &d);
        let i = calc_pos(&groups, &d, cur1, pos_to);
        let l = (i + lb).min(la) - i;
        b[..l].copy_from_slice(&a[i..(i + l)]);
        println!("s {l} {i} 0");
    }

    for &pos_to in &t {
        while cur != pos_to {
            // コスト0 で移動できる範囲で移動する
            let path = walk0(cur, pos_to, &graph, &d, &b);
            for &p in &path {
                println!("m {p}");
            }
            if let Some(&u) = path.last() {
                cur = u;
                if cur == pos_to {
                    break;
                }
            }

            // 次の中継点に向かいやすい b を得る
            let cur1 = walk1(cur, pos_to, &graph, &groups, &d);
            let i = calc_pos(&groups, &d, cur1, pos_to);
            let l = (i + lb).min(la) - i;
            b[..l].copy_from_slice(&a[i..(i + l)]);
            println!("s {l} {i} 0");
        }
    }
}
