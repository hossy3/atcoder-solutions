use proconio::input_interactive;

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
    let query = |i: usize| -> usize {
        if i <= n {
            println!("? {}", i);
            input_interactive! {
                x: usize,
            }
            x
        } else {
            0
        }
    };

    if n == 1 {
        return query(1);
    }
    let fibo = build_fibo(n);

    let mut l = 0;
    let mut rx = query(fibo[fibo.len() - 1]);
    let mut lx = query(fibo[fibo.len() - 2]);
    for v in fibo[..(fibo.len() - 1)].windows(2).rev() {
        if rx > lx {
            l += v[1];
            lx = rx;
            rx = query(l + v[1]);
        } else {
            rx = lx;
            lx = query(l + v[0]);
        }
    }

    lx.max(rx)
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
        let mut queries = vec![];
        let mut query = |i: usize| -> usize {
            if i <= n {
                queries.push(i);
                let x = a[i - 1];
                x
            } else {
                0
            }
        };

        if n == 1 {
            return (1, vec![a[0]]);
        }

        let fibo = build_fibo(n);

        let mut l = 0;
        let mut rx = query(fibo[fibo.len() - 1]);
        let mut lx = query(fibo[fibo.len() - 2]);
        for v in fibo[..(fibo.len() - 1)].windows(2).rev() {
            if rx > lx {
                l += v[1];
                lx = rx;
                rx = query(l + v[1]);
            } else {
                rx = lx;
                lx = query(l + v[0]);
            }
        }

        (lx.max(rx), queries)
    }
}
