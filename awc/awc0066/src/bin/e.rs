use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Usize1,
        t: Usize1,
        c: [usize; n],
        uv: [(Usize1, Usize1); m],
    }

    let graph = build_ungraph(n, &uv);

    let mut state = vec![vec![(0usize, 0usize); n]; 1 << n];
    state[1 << s][s] = (1, c[s]);

    let bit_test = |bits: usize, i: usize| bits & (1 << i) != 0;

    for visited in 0..(1 << n) {
        for i in 0..n {
            if state[visited][i].0 == 0 {
                continue;
            }

            for &j in &graph[i] {
                if bit_test(visited, j) {
                    continue;
                }
                let count = state[visited][i].0;
                let x = state[visited][i].1 + c[j] * count;
                let visited = visited | (1 << j);
                state[visited][j].0 += count;
                state[visited][j].1 += x;
            }
        }
    }

    let mut count = 0usize;
    let mut sum = 0usize;
    for visited in 0..(1 << n) {
        if state[visited][t].1 > 0 {
            count += state[visited][t].0;
            sum += state[visited][t].1;
        }
    }
    // eprintln!("{:?}", &state);

    let result = sum as f64 / count as f64;
    println!("{result}");
}
