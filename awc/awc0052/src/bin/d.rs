use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut s = vec![];
    let mut r = vec![];
    for _ in 0..m {
        input! {
            k0: usize,
            s0: [Usize1; k0],
            r0: u8,
        }
        let s0 = s0.iter().map(|&x| 1 << x).sum::<usize>();
        s.push(s0);
        r.push(r0);
    }

    let mut result = usize::MAX;
    for bits in 0..(1 << n) {
        if (0..m).all(|i| {
            if r[i] == 1 {
                s[i] & bits > 0
            } else {
                s[i] & bits == 0
            }
        }) {
            result = result.min(bits.count_ones() as usize);
        }
    }
    println!("{result}");
}
