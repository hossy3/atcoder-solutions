use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut dsu = Dsu::new(n);
    let mut v = vec![];
    for _ in 0..m {
        input! {
            k: usize,
            c: usize,
            a: [Usize1; k],
        }
        v.push((c, a));
    }
    v.sort();

    let mut result = 0usize;
    for (c, a) in v {
        let x = a[0];
        for &y in &a[1..] {
            if !dsu.same(x, y) {
                dsu.merge(x, y);
                result += c;
            }
        }
    }

    let yes = (1..n).all(|x| dsu.same(0, x));
    if yes {
        println!("{result}");
    } else {
        println!("-1");
    }
}
