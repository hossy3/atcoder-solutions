use proconio::input_interactive;
use std::collections::BTreeMap;

// フィボナッチ数列を n 以下の範囲で組み立てる
fn build_fibo(n: usize) -> Vec<usize> {
    if n == 1 {
        return vec![1];
    }
    let mut v = vec![1, 2];
    loop {
        let x = v[v.len() - 2] + v[v.len() - 1];
        if x > n {
            break;
        }
        v.push(x);
    }
    v
}

fn solve() -> usize {
    input_interactive! {
        n: usize,
    }
    let fibo = build_fibo(n);

    let mut map = BTreeMap::new();
    let mut query_memo = |i: usize| -> usize {
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
    for v in fibo.windows(2).rev() {
        let l0 = l + v[0];
        let r0 = l + v[1];
        if r0 > n {
            continue;
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_fibo() {
        assert_eq!(build_fibo(1), vec![1]);
        assert_eq!(build_fibo(2), vec![1, 2]);
        assert_eq!(build_fibo(3), vec![1, 2, 3]);
        assert_eq!(build_fibo(4), vec![1, 2, 3]);
        assert_eq!(build_fibo(5), vec![1, 2, 3, 5]);
    }

    #[test]
    fn test_solve_like() {
        assert_eq!(solve_like(&[1]), (1, vec![1]));
        assert_eq!(solve_like(&[1, 2]), (2, vec![2, 1]));
        assert_eq!(solve_like(&[3, 2]), (3, vec![2, 1]));
        assert_eq!(solve_like(&[1, 2, 3, 4]), (4, vec![3, 2, 4]));
        assert_eq!(solve_like(&[1, 4, 3, 2]), (4, vec![3, 2, 1]));
        assert_eq!(solve_like(&[1, 2, 3, 5, 4]), (5, vec![5, 3, 4]));
        assert_eq!(solve_like(&[1, 2, 3, 7, 6, 5, 4]), (7, vec![5, 3, 6, 4]));
    }

    fn solve_like(a: &[usize]) -> (usize, Vec<usize>) {
        let n = a.len();
        let fibo = build_fibo(n);

        let mut queries = vec![];
        let mut map = BTreeMap::new();
        let mut query_memo = |i: usize| -> usize {
            if let Some(&x) = map.get(&i) {
                return x;
            }
            queries.push(i);
            let x = a[i - 1];
            map.insert(i, x);
            x
        };

        let mut l = 0;
        for v in fibo.windows(2).rev() {
            let l0 = l + v[0];
            let r0 = l + v[1];
            if r0 > n {
                continue;
            }
            if query_memo(r0) > query_memo(l0) {
                l = l0;
            }
        }
        (query_memo(l + 1), queries)
    }
}
