use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
        q: usize,
        rcd: [(Usize1, Usize1, usize); q],
    }

    let mut m = vec![(0..w).collect::<Vec<_>>(); h]; // 同じ行で自分または右の塗られていない列番号
    let mut dsu = Dsu::new(h * w);
    for &(r, c, d) in &rcd {
        let mut score = 0;
        let r_min = r.saturating_sub(d);
        let r_max = (r + d + 1).min(h);
        for r0 in r_min..r_max {
            let d = d - r.abs_diff(r0);
            let c_min = c.saturating_sub(d);
            let c_max = (c + d + 1).min(w);
            let mut c0 = m[r0][dsu.leader(r0 * w + c_min) % w];
            while c0 < c_max {
                score += a[r0][c0];
                if c0 == w - 1 {
                    m[r0][dsu.leader(r0 * w + c0) % w] = w;
                    break;
                }
                let c1 = m[r0][dsu.leader(r0 * w + c0 + 1) % w]; // 次に塗られていない列番号
                dsu.merge(r0 * w + c0, r0 * w + c0 + 1);
                m[r0][dsu.leader(r0 * w + c0) % w] = c1;
                c0 = c1;
            }
        }
        println!("{score}");
    }
}
