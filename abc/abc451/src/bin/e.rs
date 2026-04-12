use ac_library::Dsu;
use itertools::Itertools;
use proconio::input;

fn shortest_all_graph(s: usize, graph: &[Vec<(usize, usize)>]) -> Vec<usize> {
    let n = graph.len();
    let mut v = vec![usize::MAX; n];
    v[s] = 0;
    let mut stack = vec![s];

    while let Some(i) = stack.pop() {
        for &(j, w) in &graph[i] {
            if v[j] == usize::MAX {
                v[j] = v[i] + w;
                stack.push(j);
            }
        }
    }
    v
}

fn f(a: &[Vec<usize>]) -> bool {
    let n = a.len() + 1;

    let mut v = vec![];
    for i in 0..(n - 1) {
        for j in 0..(n - i - 1) {
            v.push((a[i][j], i, i + j + 1));
        }
    }

    let mut graph = vec![vec![]; n]; // 無向グラフ
    let mut dsu = Dsu::new(n);

    for &(x, i, j) in v.iter().sorted() {
        if !dsu.same(i, j) {
            graph[i].push((j, x));
            graph[j].push((i, x));
            dsu.merge(i, j);
        }
    }

    for i in 0..(n - 1) {
        let v = shortest_all_graph(i, &graph);
        for j in 0..(n - i - 1) {
            if a[i][j] != v[i + j + 1] {
                return false;
            }
        }
    }

    true
}

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![];
    for i in 1..n {
        input! {
            a0: [usize; n - i],
        }
        a.push(a0);
    }

    let yes = f(&a);
    println!("{}", if yes { "Yes" } else { "No" });
}
