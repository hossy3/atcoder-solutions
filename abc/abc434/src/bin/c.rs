use proconio::input;

fn f(h: isize, tlu: &[(isize, isize, isize)]) -> bool {
    let mut t0 = 0;
    let mut l0 = h;
    let mut u0 = h;
    for &(t, l, u) in tlu {
        l0 = l0 - (t - t0);
        u0 = u0 + (t - t0);
        if l0 > u || u0 < l {
            return false;
        }
        t0 = t;
        l0 = l0.max(l);
        u0 = u0.min(u);
    }

    true
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            h: isize,
            tlu: [(isize, isize, isize); n],
        }
        let yes = f(h, &tlu);
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
