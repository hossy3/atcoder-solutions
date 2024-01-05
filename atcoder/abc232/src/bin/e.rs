use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
    }

    const MOD: usize = 998244353;

    let mut t = ((0usize, 0usize), (0usize, 0usize)); // ((#(x2, y2), #(!x2, y2)), (#(x2, !y2), #(!x2, !y2)))
    if x1 == x2 {
        if y1 == y2 {
            t.0 .0 = 1;
        } else {
            t.0 .1 = 1;
        }
    } else {
        if y1 == y2 {
            t.1 .0 = 1;
        } else {
            t.1 .1 = 1;
        }
    }

    for _ in 0..k {
        t = (
            (
                (t.0 .1 + t.1 .0) % MOD,
                (t.0 .0 * (w - 1) + t.1 .1 + t.0 .1 * (w - 2)) % MOD,
            ),
            (
                (t.1 .1 + t.0 .0 * (h - 1) + t.1 .0 * (h - 2)) % MOD,
                (t.1 .0 * (w - 1) + t.0 .1 * (h - 1) + t.1 .1 * ((w - 2) + (h - 2))) % MOD,
            ),
        );
    }
    println!("{}", t.0 .0);
}
