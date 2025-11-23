use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    iter,
    mem::swap,
};

use ac_library::Dsu;
use itertools::Itertools;
use proconio::input_interactive;
use rand::Rng;

fn hilbert_order(mut x: usize, mut y: usize) -> usize {
    const MAX: usize = 1 << 15;
    x = x * MAX / 20001;
    y = y * MAX / 20001;

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

fn main() {
    input_interactive! {
        n: usize,
        m: usize,
        q: usize,
        l: usize,
        _w: usize,
        g: [usize; m],
        lxrxlyry: [(usize, usize, usize, usize); n],
    }

    // それぞれの重心 (の2倍) を求め、登録する
    let mut all_nodes = vec![];
    for (i, &(lx, rx, ly, ry)) in lxrxlyry.iter().enumerate() {
        let x = lx + rx;
        let y = ly + ry;
        let amb = (rx - lx).pow(2) + (ry - ly).pow(2); // あいまい度
        all_nodes.push(((x, y), i, hilbert_order(x, y), amb));
    }

    // 位置がよくわからないノードはひとりぼっちにする
    let alone_nodes_count = g.iter().filter(|&&x| x == 1).count();
    let alone_node_ids: Vec<_> = all_nodes
        .iter()
        .sorted_by_key(|&(_, _, _, amb)| Reverse(amb))
        .take(alone_nodes_count)
        .map(|&(_, i, _, _)| i)
        .collect();

    // グループに分類する
    let mut group_nodes = vec![vec![]; m];
    {
        let mut sorted_nodes: Vec<_> = all_nodes
            .iter()
            .filter(|&(_, i, _, _)| !alone_node_ids.contains(i))
            .collect();
        sorted_nodes.sort_by_key(|&(_, _, h, _)| h);

        let mut i = 0;
        let mut i0 = 0; // ひとりぼっち用
        for (j, &count) in g.iter().enumerate() {
            if count == 1 {
                group_nodes[j].push(all_nodes[alone_node_ids[i0]].clone());
                i0 += 1;
            } else {
                for _ in 0..count {
                    group_nodes[j].push(sorted_nodes[i].clone());
                    i += 1;
                }
            }
        }
    }

    // グループごとに距離が最短となる木を作る
    let mut group_edges = vec![];
    for nodes in &group_nodes {
        let mut edges = HashSet::new();
        let mut queue = BinaryHeap::new();
        let len = nodes.len();
        let mut dsu = Dsu::new(len);
        for i in 0..(len - 1) {
            let (x0, y0) = nodes[i].0;
            for j in (i + 1)..len {
                let (x1, y1) = nodes[j].0;
                let dist = (x1.abs_diff(x0)).pow(2) + (y1.abs_diff(y0)).pow(2);
                queue.push((Reverse(dist), i, j));
            }
        }

        while let Some((Reverse(dist), i, j)) = queue.pop() {
            if dsu.same(i, j) {
                continue;
            }
            dsu.merge(i, j);
            let (_, a, _, amb0) = nodes[i];
            let (_, b, _, amb1) = nodes[j];
            let (a, b) = (a.min(b), a.max(b));
            let amb = ((amb0 + amb1) as f64).sqrt() as usize / 2;
            edges.insert((a, b, dist, amb));
        }

        group_edges.push(edges);
    }

    // 問い合わせてつなぎ変える 準備
    let mut node_to_group = vec![0; n];
    let mut group_fixed = vec![false; m];
    for (i, group) in group_nodes.iter().enumerate() {
        for &(_, j, _, _) in group {
            node_to_group[j] = i;
        }
        if group.len() <= 2 {
            group_fixed[i] = true;
        }
    }
    let mut rng = rand::thread_rng();

    // 問い合わせてつなぎ変える メイン処理
    let mut queried_count = HashMap::new();
    for _ in 0..q {
        let i = rng.gen_range(0..n);
        let group_id = node_to_group[i];
        if group_fixed[group_id] {
            continue;
        }

        // 占うノードを集める
        let mut queue = BinaryHeap::new();
        let mut query_nodes = HashSet::new();
        let mut removing_edges = vec![];
        queue.push((0, i, None));

        while let Some((_, i, edge)) = queue.pop() {
            query_nodes.insert(i);
            if let Some(edge) = edge {
                removing_edges.push(edge);
            }
            if query_nodes.len() >= l {
                break;
            }
            for &edge in &group_edges[group_id] {
                let (a, b, dist, amb) = edge;
                let k = *queried_count.get(&(a, b)).unwrap_or(&0);
                let score = (dist + amb) * 100 / (k + 2); // 何度も調べたエッジは選びにくくする
                if i == a && !query_nodes.contains(&b) {
                    queue.push((score, b, Some(edge)));
                }
                if i == b && !query_nodes.contains(&a) {
                    queue.push((score, a, Some(edge)));
                }
            }
        }

        // 占う
        let len = query_nodes.len();
        let c = query_nodes.iter().join(" ");
        println!("? {len} {c}");

        // 占い結果に差し替える
        input_interactive! {
            ab: [(usize, usize); len - 1],
        }
        for edge in removing_edges {
            group_edges[group_id].remove(&edge);
        }
        for (a, b) in ab {
            let (a, b) = (a.min(b), a.max(b));

            let ((x0, y0), _, _, amb0) = all_nodes[a];
            let ((x1, y1), _, _, amb1) = all_nodes[a];
            let amb = ((amb0 + amb1) as f64).sqrt() as usize / 2;
            let dist = (x1.abs_diff(x0)).pow(2) + (y1.abs_diff(y0)).pow(2);

            group_edges[group_id].insert((a, b, dist, amb));
        }

        let nodes: Vec<_> = query_nodes.iter().collect();
        for i in 0..(len - 1) {
            for j in (i + 1)..len {
                let (&a, &b) = (nodes[i], nodes[j]);
                let (a, b) = (a.min(b), a.max(b));
                *queried_count.entry((a, b)).or_insert(0usize) += 1;
            }
        }

        if group_nodes[group_id].len() == len {
            group_fixed[group_id] = true;
        }
    }

    // 結果を出力する
    println!("!");
    for (nodes, edges) in iter::zip(&group_nodes, &group_edges) {
        let c = nodes.iter().map(|&(_, c, _, _)| c).join(" ");
        println!("{c}");

        for &(a, b, _, _) in edges {
            println!("{a} {b}");
        }
    }
}
