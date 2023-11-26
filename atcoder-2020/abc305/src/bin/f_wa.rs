use std::{
    collections::{BTreeSet, VecDeque},
    io::{stdin, stdout, BufReader, Write},
};

use proconio::{input, marker::Usize1, source::line::LineSource};

// interactive

fn dijkstra(i: usize, graph: &[Vec<usize>], visited: &BTreeSet<usize>) -> Vec<usize> {
    let mut s = BTreeSet::new();
    let mut queue = VecDeque::new();
    s.insert(i);
    queue.push_back(vec![i]);

    while let Some(nodes) = queue.pop_front() {
        let i = *nodes.last().unwrap();
        for &i in &graph[i] {
            if s.insert(i) {
                let mut nodes = nodes.clone();
                nodes.push(i);
                if !visited.contains(&i) {
                    return nodes;
                }
                queue.push_back(nodes);
            }
        }
    }

    panic!();
}

#[test]
fn test_func() {
    assert_eq!(
        dijkstra(
            0,
            &vec![vec![1, 2, 3], vec![0], vec![0], vec![0]],
            &BTreeSet::from([0])
        ),
        vec![0, 1]
    );
    assert_eq!(
        dijkstra(
            1,
            &vec![vec![1, 2, 3], vec![0], vec![0], vec![0]],
            &BTreeSet::from([1])
        ),
        vec![1, 0]
    );
    assert_eq!(
        dijkstra(
            1,
            &vec![vec![1, 2, 3], vec![0], vec![0], vec![0]],
            &BTreeSet::from([0, 1])
        ),
        vec![1, 0, 2]
    );
}

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        n: usize,
        _: usize,
    }

    let mut graph = vec![vec![]; n];
    let mut visited = BTreeSet::new();
    let mut cur = 0;
    visited.insert(cur);

    'outer: while visited.len() < n {
        input! {
            from &mut source,
            v: [Usize1],
        }
        for &i in &v {
            if !graph[cur].contains(&i) {
                graph[cur].push(i);
                graph[i].push(cur);
            }
        }

        for &i in &graph[cur] {
            if visited.insert(i) {
                cur = i;
                println!("{}", i + 1);
                stdout().flush().unwrap();
                continue 'outer;
            }
        }

        let v = dijkstra(cur, &graph, &visited);
        for &i in &v[1..(v.len() - 1)] {
            println!("{}", i + 1);
            stdout().flush().unwrap();

            input! {
                from &mut source,
                _: [Usize1],
            }
        }

        cur = *v.last().unwrap();
        visited.insert(cur);
        println!("{}", cur + 1);
        stdout().flush().unwrap();
    }

    input! {
        from &mut source,
        _: String,
    }
}
