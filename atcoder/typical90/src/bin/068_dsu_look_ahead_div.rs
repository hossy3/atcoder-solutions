use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn init_sample(n: usize, txyv: &[(u8, usize, usize, i64)]) -> Vec<(i64, i64)> {
    let mut a = vec![None; n];
    for &(t, x, _, v) in txyv {
        if t == 0 {
            a[x] = Some(v);
        }
    }

    let mut s = vec![(0, 1); n];
    for i in 1..n {
        if let Some(a) = a[i - 1] {
            let (s0, s1) = s[i - 1];
            s[i] = (a - s0, a - s1);
        }
    }
    s
}

fn main() {
    input! {
        n: usize,
        txyv: [(u8, Usize1, Usize1, i64)],
    }

    let s = init_sample(n, &txyv);
    let mut uf = Dsu::new(n);

    for &(t, x, y, v) in &txyv {
        match t {
            0 => {
                uf.merge(x, y);
            }
            1 => {
                if uf.same(x, y) {
                    let ((x0, x1), (y0, y1)) = (s[x], s[y]);
                    let z = y0 + (v - x0) * (x1 - x0) * (y1 - y0);
                    println!("{z}");
                } else {
                    println!("Ambiguous");
                }
            }
            _ => unreachable!(),
        }
    }
}
