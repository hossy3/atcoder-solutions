use std::collections::{HashMap, HashSet, VecDeque};

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn shortest_ungraph(graph: &HashMap<usize, HashSet<usize>>, s: usize, e: usize) -> Option<usize> {
    let mut set = HashSet::new();
    let mut queue = VecDeque::new();
    set.insert(s);
    queue.push_back((0, s));

    while let Some((step, i)) = queue.pop_front() {
        if i == e {
            return Some(step);
        }
        if let Some(set0) = graph.get(&i) {
            for &j in set0 {
                if !set.contains(&j) {
                    set.insert(j);
                    queue.push_back((step + 1, j));
                }
            }
        }
    }
    None
}

fn main() {
    input! {
        h: usize,
        w: usize,
        ch: Usize1,
        cw: Usize1,
        dh: Usize1,
        dw: Usize1,
        s: [Chars; h],
    }

    let pos = |i: usize, j: usize| i * w + j;

    let mut uf = ac_library::Dsu::new(h * w);
    for i in 0..(h - 1) {
        for j in 0..w {
            if s[i][j] == '.' && s[i + 1][j] == '.' {
                uf.merge(pos(i, j), pos(i + 1, j));
            }
        }
    }
    for i in 0..h {
        for j in 0..(w - 1) {
            if s[i][j] == '.' && s[i][j + 1] == '.' {
                uf.merge(pos(i, j), pos(i, j + 1));
            }
        }
    }

    let mut graph = HashMap::new();
    for i0 in 0..h {
        for j0 in 0..w {
            let l0 = uf.leader(pos(i0, j0));
            if s[i0][j0] != '.' {
                continue;
            }
            for di in 0..=4 {
                if i0 + di < 2 || i0 + di >= h + 2 {
                    continue;
                }
                let i1 = i0 + di - 2;
                for dj in 0..=4 {
                    if j0 + dj < 2 || j0 + dj >= w + 2 {
                        continue;
                    }
                    let j1 = j0 + dj - 2;
                    if s[i1][j1] != '.' {
                        continue;
                    }
                    let l1 = uf.leader(pos(i1, j1));
                    if l0 == l1 {
                        continue;
                    }
                    graph.entry(l0).or_insert(HashSet::new()).insert(l1);
                    graph.entry(l1).or_insert(HashSet::new()).insert(l0);
                }
            }
        }
    }

    let result = shortest_ungraph(&graph, uf.leader(pos(ch, cw)), uf.leader(pos(dh, dw)));
    if let Some(result) = result {
        println!("{result}");
    } else {
        println!("-1");
    }
}
