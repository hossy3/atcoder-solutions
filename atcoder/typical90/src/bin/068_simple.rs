use proconio::{input, marker::Usize1};

fn f((x, y, v): (usize, usize, i64), a: &[Option<i64>]) -> Option<i64> {
    let mut z = v;
    if x < y {
        for j in x..y {
            let Some(z0) = a[j] else { 
                return None;
            };
            z = z0 - z;
        }
    } else if x > y {
        for j in (y..x).rev() {
            let Some(z0) = a[j] else { 
                return None;
            };
            z = z0 - z;
        }
    }
    Some(z)
}

fn main() {
    input! {
        n: usize,
        txyv: [(u8, Usize1, Usize1, i64)],
    }
    let mut a = vec![None; n];
    for &(t, x, y, v) in &txyv {
        match t {
            0 => {
                a[x] = Some(v);
            },
            1 => {
                if let Some(z) = f((x, y, v), &a) {
                    println!("{z}");
                } else {
                    println!("Ambiguous");
                }
            },
            _ => unreachable!(),
        }
    }
}
