use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        lr: [(usize, usize); n],
    }

    let mut result = 0;
    let mut l2r = vec![m + 1; m + 1];
    let mut r2l = vec![0; m + 1];
    for (l, r) in lr {
        l2r[l] = l2r[l].min(r);
        r2l[r] = r2l[r].max(l);
    }

    for l in (0..m).rev() {
        l2r[l] = l2r[l].min(l2r[l + 1]);
    }
    for r in 0..m {
        r2l[r] = r2l[r].max(r2l[r + 1]);
    }

    let mut r = 1;
    for l in 1..=m {
        while r < l || (r <= m && (l2r[l] > r || r2l[r] < l)) {
            r += 1;
        }
        result += r - l;
    }
    println!("{result}");
}
