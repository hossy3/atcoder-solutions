use proconio::{input, marker::Usize1};

fn f(k: usize, i: usize, l: usize) -> usize {
    l / k + if i < l % k { 1 } else { 0 }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
        q: usize,
        lr: [(Usize1, Usize1); q],
    }

    let mut acc = vec![vec![0i64; n / k + 2]; k];
    for i in 0..n {
        let j0 = i % k;
        let j1 = i / k;
        acc[j0][j1 + 1] = acc[j0][j1] + a[i];
    }

    for (l, r) in lr {
        let mut v = vec![0i64; k];
        for i in 0..k {
            let r0 = f(k, i, r + 1);
            let l0 = f(k, i, l);
            v[i] = acc[i][r0] - acc[i][l0];
        }
        let yes = v[1..].iter().all(|&x| x == v[0]);
        println!("{}", if yes { "Yes" } else { "No" });
    }
}


#[test]
fn test_f() {
    assert_eq!(f(5, 0, 0), 0);
    assert_eq!(f(5, 1, 0), 0);
    assert_eq!(f(5, 2, 0), 0);
    assert_eq!(f(5, 3, 0), 0);
    assert_eq!(f(5, 4, 0), 0);

    assert_eq!(f(5, 0, 1), 1);
    assert_eq!(f(5, 1, 1), 0);
    assert_eq!(f(5, 2, 1), 0);
    assert_eq!(f(5, 3, 1), 0);
    assert_eq!(f(5, 4, 1), 0);

    assert_eq!(f(5, 0, 13), 3);
    assert_eq!(f(5, 1, 13), 3);
    assert_eq!(f(5, 2, 13), 3);
    assert_eq!(f(5, 3, 13), 2);
    assert_eq!(f(5, 4, 13), 2);
}
