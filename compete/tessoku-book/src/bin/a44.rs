use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let f = |i: usize, dir: bool, n: usize| if dir { i } else { n - i - 1 };

    let mut a = Vec::with_capacity(n);
    for i in 0..n {
        a.push(i + 1);
    }
    let mut dir = true;
    for _ in 0..q {
        input! {
            k: usize,
        }
        match k {
            1 => {
                input! {
                    x: Usize1,
                    y: usize,
                }
                a[f(x, dir, n)] = y;
            }
            2 => {
                dir = !dir;
            }
            _ => {
                input! {
                    x: Usize1,
                }
                println!("{}", a[f(x, dir, n)]);
            }
        }
    }
}
