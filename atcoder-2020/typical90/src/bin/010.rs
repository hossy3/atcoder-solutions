use proconio::input;

fn main() {
    input! {
        n: usize,
        cp: [(usize, usize); n],
        q: usize,
        lr: [(usize, usize); q],
    }
    let mut cum = vec![(0usize, 0usize); n + 1];
    for i in 0..n {
        let (c, p) = cp[i];
        cum[i + 1].0 = cum[i].0;
        cum[i + 1].1 = cum[i].1;
        if c == 1 {
            cum[i + 1].0 += p;
        } else if c == 2 {
            cum[i + 1].1 += p;
        }
    }

    for (l, r) in lr {
        let x = cum[r].0 - cum[l - 1].0;
        let y = cum[r].1 - cum[l - 1].1;
        println!("{} {}", x, y);
    }
}
