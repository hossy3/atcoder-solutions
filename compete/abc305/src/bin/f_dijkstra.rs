use std::{
    collections::VecDeque,
    io::{stdin, stdout, BufReader, Write},
};

use proconio::{input, marker::Usize1, source::line::LineSource};

fn dijkstra(i: usize, graph: &[Vec<usize>], visited: &[bool]) -> Vec<usize> {
    let mut v = vec![false; graph.len()];
    let mut queue = VecDeque::new();
    v[i] = true;
    queue.push_back(vec![i]);

    while let Some(nodes) = queue.pop_front() {
        let i = *nodes.last().unwrap();
        for &i in &graph[i] {
            if !v[i] {
                v[i] = true;
                let mut nodes = nodes.clone();
                nodes.push(i);
                if !visited[i] {
                    return nodes;
                }
                queue.push_back(nodes);
            }
        }
    }

    panic!();
}

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        n: usize,
        _: usize,
    }

    let mut graph = vec![vec![]; n + 1];
    let mut visited = vec![false; n + 1];
    let mut cur = 1;
    visited[cur] = true;

    loop {
        input! {
            from &mut source,
            v: [usize],
        }
        if v.contains(&n) {
            break;
        }
        for &i in &v {
            if !graph[cur].contains(&i) {
                graph[cur].push(i);
                graph[i].push(cur);
            }
        }

        if let Some(&i) = graph[cur].iter().find(|&&i| !visited[i]) {
            visited[i] = true;
            cur = i;
            println!("{}", i);
        } else {
            let v = dijkstra(cur, &graph, &visited);
            for &i in &v[1..(v.len() - 1)] {
                println!("{}", i);
                stdout().flush().unwrap();

                input! {
                    from &mut source,
                    _: [Usize1],
                }
            }

            cur = *v.last().unwrap();
            visited[cur] = true;
            println!("{}", cur);
        }
        stdout().flush().unwrap();
    }

    println!("{}", n);
    stdout().flush().unwrap();

    input! {
        from &mut source,
        s: String,
    }
    assert_eq!(s, "OK");
}
