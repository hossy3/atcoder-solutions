use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: u32,
        m: usize,
        s: [isize; n],
        lrp: [(Usize1, Usize1, isize); m],
    }

    // 全通り試す
    let mut max = 0;
    for bits in 0usize..(1 << n) {
        if bits.count_ones() > k {
            continue;
        }
        let mut sum = 0;
        for i in 0..n {
            if bits & (1 << i) != 0 {
                sum += s[i];
            }
        }
        for &(l, r, p) in &lrp {
            let bits0 = (1 << (r + 1)) - (1 << l);
            if bits & bits0 != 0 {
                sum += p;
            }
        }
        max = max.max(sum);
    }
    println!("{max}");
}
