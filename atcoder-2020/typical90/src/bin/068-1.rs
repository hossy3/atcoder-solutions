use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        txyv: [(usize, Usize1, Usize1, i64)],
    }
    let mut a = vec![-1; n];
    'outer: for &(t, x, y, v) in &txyv {
        if t == 0 {
            a[x] = v;
        } else {
            let mut z = v;
            if x < y {
                for j in x..y {
                    if a[j] == -1 {
                        println!("Ambiguous");
                        continue 'outer;
                    }
                    z = a[j] - z;
                }
            } else if x > y {
                for j in (y..x).rev() {
                    if a[j] == -1 {
                        println!("Ambiguous");
                        continue 'outer;
                    }
                    z = a[j] - z;
                }
            }
            println!("{}", z);
        }
    }
}
