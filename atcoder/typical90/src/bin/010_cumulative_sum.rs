use proconio::input;

fn main() {
    input! {
        n: usize,
        cp: [(usize, usize); n],
        q: usize,
        lr: [(usize, usize); q],
    }

    let mut cum = vec![(0usize, 0usize); n + 1];
    for (i, &(c, p)) in cp.iter().enumerate() {
        cum[i + 1].0 = cum[i].0;
        cum[i + 1].1 = cum[i].1;
        match c {
            1 => {
                cum[i + 1].0 += p;
            }
            2 => {
                cum[i + 1].1 += p;
            }
            _ => unreachable!(),
        }
    }

    for (l, r) in lr {
        let x = cum[r].0 - cum[l - 1].0;
        let y = cum[r].1 - cum[l - 1].1;
        println!("{x} {y}");
    }
}
