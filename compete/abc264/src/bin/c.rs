use proconio::input;

fn main() {
    input! {
        h1: usize,
        w1: usize,
        a: [[usize; w1]; h1],
        h2: usize,
        w2: usize,
        b: [[usize; w2]; h2],
    }

    for h in 0..(1 << h1) {
        let h = h as usize;
        if h.count_ones() != h2 as u32 {
            continue;
        }
        let mut hs = vec![0; h2];
        let mut j = 0;
        for i in 0..h1 {
            if ((1 << i) & h) > 0 {
                hs[j] = i;
                j += 1;
            }
        }
        for w in 0..(1 << w1) {
            let w = w as usize;
            if w.count_ones() != w2 as u32 {
                continue;
            }
            let mut ws = vec![0; w2];
            let mut j = 0;
            for i in 0..w1 {
                if ((1 << i) & w) > 0 {
                    ws[j] = i;
                    j += 1;
                }
            }

            let mut yes = true;
            'inner: for i in 0..h2 {
                for j in 0..w2 {
                    if a[hs[i]][ws[j]] != b[i][j] {
                        yes = false;
                        break 'inner;
                    }
                }
            }
            if yes {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
