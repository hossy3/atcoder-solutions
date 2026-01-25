use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
    }

    let mut cum = vec![0usize; n + 1];
    for (i, &x) in a.iter().enumerate() {
        cum[i + 1] = cum[i] + x;
    }

    for _ in 0..q {
        input! {
            q: u8,
        }
        match q {
            1 => {
                input! {
                    x: Usize1,
                }
                a.swap(x, x + 1);
                cum[x + 1] += a[x];
                cum[x + 1] -= a[x + 1];
            }
            2 => {
                input! {
                    l: Usize1,
                    r: Usize1,
                }
                let result = cum[r + 1] - cum[l];
                println!("{result}");
            }
            _ => unreachable!(),
        }
    }
}
