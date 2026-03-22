use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; n],
        q: usize,
        x: [Usize1; q],
    }

    let mut v = vec![0usize; m];
    for &x in &a {
        v[x] += 1;
    }
    let mut v0 = vec![vec![]; n + 1];
    for (i, &x) in v.iter().enumerate() {
        v0[x].push(i); // x個存在する
    }

    let mut results = vec![];
    'outer: for mut x in x {
        if x < a.len() {
            results.push(a[x] + 1);
            continue;
        }
        x -= a.len();

        let mut len = 0usize;
        for i in 0..=n {
            len += v0[i].len();
            if x < len {
                for j in 0..m {
                    if v[j] <= i {
                        if x == 0 {
                            results.push(j + 1);
                            continue 'outer;
                        }
                        x -= 1;
                    }
                }
                unreachable!();
            }
            x -= len;
        }

        // すべて使う
        x = x % m;
        results.push(x + 1);
    }

    for result in results {
        println!("{result}");
    }
}
