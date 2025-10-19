use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let mut cum = vec![0; 2 * n + 1]; // 累積和 2周分
    for i in 0..(2 * n) {
        cum[i + 1] = cum[i] + a[i % n];
    }

    let mut offset = 0; // 元の offset 番目が今の先頭になる
    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {
                    c: usize,
                }
                offset = (offset + c) % n;
            }
            2 => {
                input! {
                    l: Usize1,
                    r: Usize1,
                }
                let (l, r) = (l + offset, r + offset);
                let result = cum[r + 1] - cum[l];
                println!("{result}");
            }
            _ => unreachable!(),
        }
    }
}
