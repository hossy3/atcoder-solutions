use proconio::input;

// heuristic - greedy

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n]
    }

    let mut result = Vec::with_capacity(n + 1);
    result.push(0);
    let mut visited = vec![false; n];
    visited[0] = true;

    for _ in 1..n {
        let prev = *result.last().unwrap();
        let i = (1..n)
            .filter(|&i| !visited[i])
            .min_by_key(|&i| (xy[prev].0 - xy[i].0).pow(2) + (xy[prev].1 - xy[i].1).pow(2))
            .unwrap();
        visited[i] = true;
        result.push(i);
    }
    result.push(0);

    for &i in &result {
        println!("{}", i + 1);
    }
}
