use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        q: usize,
    }

    let mut v = vec![];
    let mut sorted = false;
    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
        }

        match t {
            1 => {
                v.push(x);
                sorted = false;
            }
            2 => {
                input! {
                    k: usize,
                }
                if !sorted {
                    v.sort();
                    sorted = true;
                }
                let i = v.upper_bound(&x);
                if v.is_empty() || i < k {
                    println!("{}", -1);
                } else {
                    println!("{}", v[i - k]);
                }
            }
            _ => {
                input! {
                    k: usize,
                }
                if !sorted {
                    v.sort();
                    sorted = true;
                }
                let i = v.lower_bound(&x);
                if i + k - 1 >= v.len() {
                    println!("{}", -1);
                } else {
                    println!("{}", v[i + k - 1]);
                }
            }
        }
    }
}
