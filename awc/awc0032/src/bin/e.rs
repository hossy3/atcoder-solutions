use ac_library::FenwickTree;
use proconio::input;

const N: usize = 100;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: [usize; n],
    }

    let mut fenwick = FenwickTree::new(n, 0);
    for (i, &s) in s.iter().enumerate() {
        fenwick.add(i, s);
    }
    let mut a = vec![0; N + 1]; // N 以下は倍数が何回現れたかを確認

    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {
                    k: usize,
                    v: usize,
                }
                if k <= N {
                    a[k] += v;
                } else {
                    for i in ((k - 1)..n).step_by(k) {
                        fenwick.add(i, v);
                    }
                }
            }
            2 => {
                input! {
                    x: usize,
                }
                let mut result = fenwick.sum(..x);
                for i in 1..=N {
                    result += a[i] * (x / i);
                }
                println!("{result}");
            }
            _ => unreachable!(),
        }
    }
}
