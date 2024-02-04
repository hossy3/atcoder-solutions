use ac_library::Dsu;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }

    let f = |r: usize, c: usize| -> usize { r * (w + 2) + c };

    let mut v = vec![false; (h + 2) * (w + 2)];
    let mut uf = Dsu::new((h + 2) * (w + 2));

    for _ in 0..q {
        input! {
            k: u8,
        }
        match k {
            1 => {
                input! {
                    r: usize,
                    c: usize,
                }
                let i = f(r, c);
                v[i] = true;
                let dir: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                for &(dr, dc) in &dir {
                    let (r0, c0) = (r.wrapping_add_signed(dr), c.wrapping_add_signed(dc));
                    if v[f(r0, c0)] {
                        uf.merge(i, f(r0, c0));
                    }
                }
            }
            2 => {
                input! {
                    ra: usize,
                    ca: usize,
                    rb: usize,
                    cb: usize,
                }

                let yes = v[f(ra, ca)] && v[f(rb, cb)] && uf.same(f(ra, ca), f(rb, cb));
                println!("{}", if yes { "Yes" } else { "No" });
            }
            _ => unreachable!(),
        }
    }
}
