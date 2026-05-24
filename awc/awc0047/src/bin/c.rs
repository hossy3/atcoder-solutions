use ac_library::FenwickTree;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [isize; n],
    }

    let mut imos = vec![0isize; n + 1];
    imos[0] = a[0];
    for i in 0..(n - 1) {
        imos[i + 1] = a[i + 1] - a[i]; // 累積和が a になるようにする
    }
    // eprintln!("{:?}", &imos);
    let mut fenwick = FenwickTree::new(n + 1, 0isize);
    for (i, &x) in imos.iter().enumerate() {
        fenwick.add(i, x);
    }

    for _ in 0..q {
        input! {
            k: u8,
        }
        match k {
            1 => {
                input! {
                    l: usize,
                    r: usize,
                    x: isize,
                }
                fenwick.add(l - 1, x);
                fenwick.add(r, -x);
            }
            2 => {
                input! {
                    p: usize,
                }
                let result = fenwick.accum(p);
                println!("{result}");
            }
            _ => unreachable!(),
        }
    }
}
