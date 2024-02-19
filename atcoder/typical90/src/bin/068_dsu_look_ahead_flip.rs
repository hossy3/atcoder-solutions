use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn init_sample(n: usize, txyv: &[(u8, usize, usize, i64)]) -> Vec<i64> {
    let mut a = vec![None; n];
    for &(t, x, _, v) in txyv {
        if t == 0 {
            a[x] = Some(v);
        }
    }

    let mut s = vec![0; n];
    for i in 1..n {
        if let Some(a) = a[i - 1] {
            let s0 = s[i - 1];
            s[i] = a - s0;
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
                    let z = if y.abs_diff(x) % 2 == 0 {
                        s[y] + (v - s[x])
                    } else {
                        s[y] - (v - s[x])
                    };
                    println!("{z}");
                } else {
                    println!("Ambiguous");
                }
            }
            _ => unreachable!(),
        }
    }
}
