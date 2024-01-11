use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn f(w: usize, r: usize, c: usize) -> usize {
    r * w + c
}

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }

    let mut m = vec![vec![false; w]; h];
    let mut uf = Dsu::new(h * w);

    for _ in 0..q {
        input! {
            k: u8,
        }
        match k {
            1 => {
                input! {
                    r: Usize1,
                    c: Usize1,
                }
                m[r][c] = true;
                if r > 0 && m[r - 1][c] {
                    uf.merge(f(w, r, c), f(w, r - 1, c));
                }
                if r < (h - 1) && m[r + 1][c] {
                    uf.merge(f(w, r, c), f(w, r + 1, c));
                }
                if c > 0 && m[r][c - 1] {
                    uf.merge(f(w, r, c), f(w, r, c - 1));
                }
                if c < (w - 1) && m[r][c + 1] {
                    uf.merge(f(w, r, c), f(w, r, c + 1));
                }
            }
            2 => {
                input! {
                    ra: Usize1,
                    ca: Usize1,
                    rb: Usize1,
                    cb: Usize1,
                }

                let yes = m[ra][ca] && m[rb][cb] && uf.same(f(w, ra, ca), f(w, rb, cb));
                println!("{}", if yes { "Yes" } else { "No" });
            }
            _ => unreachable!(),
        }
    }
}
