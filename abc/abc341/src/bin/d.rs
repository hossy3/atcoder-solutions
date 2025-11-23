use num_integer::Integer;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, k): (usize, usize, Usize1),
    }

    let lcm = n.lcm(&m);
    let nset = lcm / n + lcm / m - 2; // lcm 1周で消費する個数

    let mut result = (k / nset) * lcm;
    let k = k % nset;

    let mut n0 = result + n;
    let mut m0 = result + m;
    for _ in 0..=k {
        if n0 < m0 {
            result = n0;
            n0 += n;
        } else {
            result = m0;
            m0 += m;
        }
    }

    println!("{result}");
}
