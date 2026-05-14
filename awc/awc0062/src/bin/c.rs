use proconio::input;

trait BitTest {
    fn bit_test(&self, i: usize) -> bool;
}

impl BitTest for usize {
    fn bit_test(&self, i: usize) -> bool {
        self & (1 << i) != 0
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        d: usize,
        ab: [(usize, usize); n],
    }

    let mut result = 0;
    for bits in 0..(1 << n) {
        let mut a0 = 0;
        let mut b0 = 0;
        for i in 0..n {
            if bits.bit_test(i) {
                a0 += ab[i].0;
                b0 += ab[i].1;
            }
        }
        result = result.max(a0.saturating_sub(b0.saturating_sub(k) * d));
    }
    println!("{result}");
}
