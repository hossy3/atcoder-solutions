use pathfinding::directed::dijkstra::dijkstra_all;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut graph = vec![vec![]; n + m];
    for i in 0..m {
        let j = n + i;

        input! {
            k: usize,
            r: [Usize1; k],
        }
        for x in r {
            graph[j].push(x);
            graph[x].push(j);
        }
    }

    let reachable = dijkstra_all(&0, |&i| graph[i].iter().map(|&x| (x, 1usize)));
    println!("0");
    for i in 1..n {
        if let Some(&(_, step)) = reachable.get(&i) {
            let result = step / 2;
            println!("{result}");
        } else {
            println!("-1");
        }
    }
}
