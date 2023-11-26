use itertools::Itertools;
use proconio::{input, marker::Chars};

fn f(m: &[Vec<usize>], i: usize) -> usize {
    for v in m {
        if v[i] > 0 {
            return v[i];
        }
    }
    0
}

fn g(m: &[Vec<usize>], i: usize) -> bool {
    let mut a = [false; 4];
    for v in m {
        if v[i] > 0 && a[v[i]] {
            return false;
        }
        a[v[i]] = true;
    }
    true
}

fn conv(c: char) -> usize {
    match c {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => unreachable!(),
    }
}

fn func(n: usize, r: &[char], c: &[char], i: usize, m: &mut [Vec<usize>]) -> bool {
    for v in (0..n).permutations(3) {
        if (v[0] < v[1] && v[0] < v[2] && r[i] != 'A')
            || (v[1] < v[0] && v[1] < v[2] && r[i] != 'B')
            || (v[2] < v[0] && v[2] < v[1] && r[i] != 'C')
        {
            continue;
        }
        let c0 = f(&m[..i], v[0]);
        let c1 = f(&m[..i], v[1]);
        let c2 = f(&m[..i], v[2]);
        if (c0 != 0 && c0 != conv(c[v[0]]))
            || (c1 != 0 && c1 != conv(c[v[1]]))
            || (c2 != 0 && c2 != conv(c[v[2]]))
        {
            continue;
        }
        for j in 0..n {
            m[i][j] = 0;
        }
        m[i][v[0]] = 1;
        m[i][v[1]] = 2;
        m[i][v[2]] = 3;
        if !g(&m[..=i], v[0]) || !g(&m[..=i], v[1]) || !g(&m[..=i], v[2]) {
            continue;
        }
        if i == n - 1 {
            return true;
        } else {
            let b = func(n, r, c, i + 1, m);
            if b {
                return true;
            }
        }
    }

    false
}

fn main() {
    input! {
        n: usize,
        r: Chars,
        c: Chars,
    }
    let mut m = vec![vec![0; n]; n];
    let result = func(n, &r, &c, 0, &mut m);

    if result {
        println!("Yes");
        for v in m {
            let line = v
                .iter()
                .map(|i| match i {
                    0 => '.',
                    1 => 'A',
                    2 => 'B',
                    3 => 'C',
                    _ => unreachable!(),
                })
                .into_iter()
                .join("");
            println!("{line}");
        }
    } else {
        println!("No");
    }
}
