use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Usize1; n],
        tb: [(usize, Usize1); q],
    }

    // まずループを調べる
    let mut v = vec![(usize::MAX, 0); n]; // MAX は未探索, 0 はループなし
    for i in 0..n {
        if v[i].0 < usize::MAX {
            continue; // ループを調べている
        }

        let mut path = vec![i];
        let mut prev = vec![usize::MAX; n];
        prev[i] = 0;
        let mut x = a[i];

        while prev[x] == usize::MAX && v[x].0 == usize::MAX {
            prev[x] = path.len();
            path.push(x);
            x = a[x];
        }

        if prev[x] < usize::MAX {
            let x = prev[x];
            for &y in &path[0..x] {
                v[y] = (0, 0);
            }
            let sum: usize = path[x..].iter().map(|&x| x + 1).sum();
            for &y in &path[x..] {
                v[y] = (path.len() - x, sum);
            }
        } else {
            for &y in &path {
                v[y] = (0, 0);
            }
        }
    }

    // ループを使って解答する
    for &(mut t, mut b) in &tb {
        let mut result = 0;
        while t > 0 {
            if v[b].0 > 0 && t >= v[b].0 {
                let x = t / v[b].0; // ループ回数
                t -= x * v[b].0;
                result += x * v[b].1;
            }
            if t > 0 {
                t -= 1;
                result += b + 1;
                b = a[b];
            }
        }
        println!("{result}");
    }
}
