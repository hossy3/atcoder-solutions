use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn build_digraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
    }
    graph
}

fn update(
    graph: &[Vec<usize>],
    i: usize,
    cur: &mut Vec<usize>,
    tops: &mut BTreeSet<usize>,
    nup: &mut Vec<usize>,
) {
    cur.push(i);
    tops.remove(&i);
    for &j in &graph[i] {
        nup[j] -= 1;
        if nup[j] == 0 {
            tops.insert(j);
        }
    }
}

fn f(graph: &[Vec<usize>], k: usize) -> Vec<Vec<usize>> {
    let n = graph.len();
    let mut results = vec![];

    let mut nup = vec![0usize; n];
    for v in graph {
        for &i in v {
            nup[i] += 1;
        }
    }

    let tops: BTreeSet<_> = (0..n).filter(|&i| nup[i] == 0).collect();
    let mut stack = vec![];
    for &i in &tops {
        let mut cur = vec![];
        let (mut tops, mut nup) = (tops.clone(), nup.clone());
        update(&graph, i, &mut cur, &mut tops, &mut nup);
        stack.push((cur, tops, nup));
        if stack.len() + results.len() == k {
            break;
        }
    }

    while let Some((mut cur, mut tops, mut nup)) = stack.pop() {
        if cur.len() == n {
            results.push(cur);
            continue;
        }

        let ntops = tops.len();
        if ntops == 0 {
            break;
        } else if ntops == 1 || stack.len() + results.len() + 1 == k {
            let Some(&i) = tops.first() else { unreachable!() };
            update(&graph, i, &mut cur, &mut tops, &mut nup);
            stack.push((cur, tops, nup));
        } else {
            for &i in &tops {
                let (mut cur, mut tops, mut nup) = (cur.clone(), tops.clone(), nup.clone());
                update(&graph, i, &mut cur, &mut tops, &mut nup);
                stack.push((cur, tops, nup));
                if stack.len() + results.len() == k {
                    break;
                }
            }
        }
    }

    results
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        ab: [(Usize1, Usize1); m],
    }

    let graph = build_digraph(n, &ab);
    let results = f(&graph, k);
    if results.len() == k {
        for results in results {
            let result = results.iter().map(|&i| i + 1).join(" ");
            println!("{result}");
        }
    } else {
        println!("-1");
    }
}
