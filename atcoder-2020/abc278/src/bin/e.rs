use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        _: usize,
        h0: usize,
        w0: usize,
        a: [[Usize1; w]; h],
    }

    let mut m0 = vec![0; 300];
    for h1 in 0..h {
        for w1 in 0..w {
            if h1 >= h0 || w1 >= w0 {
                m0[a[h1][w1]] += 1;
            }
        }
    }

    for h1 in 0..=(h - h0) {
        if h1 > 0 {
            for w1 in 0..w0 {
                m0[a[h1 - 1][w1]] += 1;
                m0[a[h1 + h0 - 1][w1]] -= 1;
            }
        }
        let mut m1 = m0.clone();
        for w1 in 0..=(w - w0) {
            if w1 > 0 {
                for h2 in h1..(h1 + h0) {
                    m1[a[h2][w1 - 1]] += 1;
                    m1[a[h2][w1 + w0 - 1]] -= 1;
                }
            }
            let mut score = 0;
            for &x in &m1 {
                if x > 0 {
                    score += 1;
                }
            }
            if w1 == w - w0 {
                println!("{}", score);
            } else {
                print!("{} ", score);
            }
        }
    }
}
