use ac_library::FenwickTree;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut s: [isize; n],
    }

    let mut fenwick = FenwickTree::new(n, 0isize);
    for (i, &x) in s.iter().enumerate() {
        fenwick.add(i, x);
    }

    for _ in 0..q {
        input! {
            t: u8,
        }

        match t {
            1 => {
                input! {
                    l: Usize1,
                    r: Usize1,
                }
                let result = fenwick.sum(l..=r);
                println!("{result}");
            }
            2 => {
                input! {
                    x: Usize1,
                    v: isize,
                }
                let diff = v - s[x];
                fenwick.add(x, diff);
                s[x] = v;
            }
            _ => unreachable!(),
        }
    }
}
