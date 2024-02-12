use proconio::{input, marker::Usize1};

// a, b どちらかが白いときに i を黒くできる
// 逆順に考えると、a を白くすると i も白くできる

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n],
    }

    let mut graph = vec![vec![]; n];
    for (i, &(a, b)) in ab.iter().enumerate() {
        graph[a].push(i);
        graph[b].push(i);
    }

    let mut stack = vec![];
    let mut visited = vec![false; n];
    for (i, &(a, b)) in ab.iter().enumerate() {
        if i == a || i == b {
            stack.push(i);
            visited[i] = true;
        }
    }

    let mut path = vec![];
    while let Some(i) = stack.pop() {
        path.push(i);

        for &i in &graph[i] {
            if visited[i] {
                continue;
            }
            visited[i] = true;
            stack.push(i);
        }
    }

    if path.len() == n {
        for &x in path.iter().rev() {
            println!("{}", x + 1);
        }
    } else {
        println!("-1");
    }
}
