use std::collections::BTreeSet;

use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut xy: [(usize, usize); n],
    }

    let mut dsu = Dsu::new(n + q);

    // 全部の組み合わせの距離を登録しておく
    let mut set = BTreeSet::new();
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let (x0, y0) = xy[i];
            let (x1, y1) = xy[j];
            let dist = x0.abs_diff(x1) + y0.abs_diff(y1);
            set.insert((dist, (i, j)));
        }
    }

    'outer: for _ in 0..q {
        input! {
            t: usize,
        }
        // eprintln!("{}: {}", t, &dsu.groups().len());
        match t {
            1 => {
                input! {
                    a: usize,
                    b: usize,
                }
                let j = xy.len();
                for i in 0..j {
                    let (x0, y0) = xy[i];
                    let dist = x0.abs_diff(a) + y0.abs_diff(b);
                    set.insert((dist, (i, j)));
                }
                xy.push((a, b));
            }
            2 => {
                while let Some((dist, (i, j))) = set.pop_first() {
                    if dsu.same(i, j) {
                        continue;
                    }
                    dsu.merge(i, j);
                    println!("{dist}");
                    while let Some((dist0, (i0, j0))) = set.pop_first() {
                        if dist0 > dist {
                            set.insert((dist0, (i0, j0)));
                            break;
                        }
                        dsu.merge(i0, j0);
                    }
                    continue 'outer;
                }
                println!("-1");
            }
            3 => {
                input! {
                    u: Usize1,
                    v: Usize1,
                }
                let yes = dsu.same(u, v);
                println!("{}", if yes { "Yes" } else { "No" });
            }
            _ => unreachable!(),
        }
    }
}
