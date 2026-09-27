use std::collections::VecDeque;

use ac_library::Dsu;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn shortest_all_ungraph(s: usize, graph: &[Vec<usize>]) -> Vec<Option<usize>> {
    let n = graph.len();
    let mut v = vec![None; n];
    let mut queue = VecDeque::new();
    v[s] = Some(0usize);
    queue.push_back((0, s));

    while let Some((step, i)) = queue.pop_front() {
        for &j in &graph[i] {
            if v[j] == None {
                v[j] = Some(step + 1);
                queue.push_back((step + 1, j));
            }
        }
    }
    v
}

fn main() {
    input! {
        t: usize,
    }

    'outer: for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            ab: [(Usize1, Usize1); m],
        }

        let mut graph = vec![vec![]; n]; // n頂点のグラフ
        let mut dsu = Dsu::new(n * 2);
        for (a, b) in ab {
            dsu.merge(a * 2, b * 2 + 1);
            dsu.merge(a * 2 + 1, b * 2);
            if dsu.same(a * 2, a * 2 + 1) {
                // 2部グラフでないので、A から B への奇数辺の最短経路があるはず
                let v = shortest_all_ungraph(a, &graph);
                let mut results = vec![b];
                while results[results.len() - 1] != a {
                    let b = results[results.len() - 1];
                    let step = v[b].unwrap();
                    for &b in &graph[b] {
                        if let Some(step0) = v[b] {
                            if step0 == step - 1 {
                                results.push(b);
                                break;
                            }
                        }
                    }
                    // eprintln!("{:?}", &results);
                }

                println!("{}", results.len());
                println!("{}", results.iter().map(|&x| x + 1).join(" "));
                continue 'outer;
            }

            graph[a].push(b);
            graph[b].push(a);
        }

        println!("-1");
    }
}
