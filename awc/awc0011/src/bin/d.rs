use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        v: [usize; n],
        p: [Usize1; n - 1],
        x: [Usize1; q],
    }

    // 親から子をたどるグラフ
    let mut children = vec![vec![]; n];
    for (i, &x) in p.iter().enumerate() {
        children[x].push(i + 1);
    }

    // 遺産を親からたどる
    let mut results = vec![0usize; n];
    let mut stack = vec![(0, v[0])];
    while let Some((i, sum)) = stack.pop() {
        results[i] = sum;
        for &j in &children[i] {
            stack.push((j, sum + v[j]));
        }
    }

    // 答え
    for &i in &x {
        println!("{}", results[i]);
    }
}
