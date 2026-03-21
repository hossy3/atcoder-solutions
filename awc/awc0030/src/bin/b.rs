use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        pt: [(usize, u8); n],
    }

    for _ in 0..m {
        input! {
            k: usize,
            s: [Usize1; k],
        }

        let mut p0 = usize::MAX;
        let mut p1 = usize::MAX;
        for i in s {
            let (p, t) = pt[i];
            if t == 0 {
                p0 = p0.min(p);
            } else {
                p1 = p1.min(p);
            }
        }
        if p0 == usize::MAX || p1 == usize::MAX {
            println!("-1");
        } else {
            println!("{}", p0 + p1);
        }
    }
}
