use proconio::input_interactive;
use std::collections::BTreeMap;

// フィボナッチ数列を n 以上の値が現れるまで組み立てる
fn build_fibo(n: usize) -> Vec<usize> {
    if n == 1 {
        return vec![1];
    }
    let mut v = vec![1, 2];
    while v[v.len() - 1] < n {
        let x = v[v.len() - 2] + v[v.len() - 1];
        v.push(x);
    }
    v
}

fn solve() -> usize {
    input_interactive! {
        n: usize,
    }
    let fibo = build_fibo(n + 1);

    let mut map = BTreeMap::new();
    let mut query_memo = |i: usize| -> usize {
        let i = i.min(n);
        if let Some(&x) = map.get(&i) {
            return x;
        }

        println!("? {}", i);
        input_interactive! {
            x: usize,
        }
        map.insert(i, x);
        x
    };

    let mut l = 0;
    for v in fibo.windows(3).rev() {
        let l0 = l + v[0];
        let r0 = l + v[1];
        if query_memo(r0) > query_memo(l0) {
            l = l0;
        }
    }
    query_memo(l + 1)
}

fn main() {
    input_interactive! {
        t: usize,
    }
    for _ in 0..t {
        let x = solve();
        println!("! {x}");
    }
}
