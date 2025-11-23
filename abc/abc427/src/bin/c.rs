use proconio::{input, marker::Usize1};

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
        m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut result = usize::MAX;
    for k in 0..(1 << n) {
        let mut count = 0;
        for &(u, v) in &uv {
            if k.bit_test(u) == k.bit_test(v) {
                count += 1;
            }
        }
        result = result.min(count);
    }
    println!("{result}");
}
