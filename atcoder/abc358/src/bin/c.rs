use proconio::{input, marker::Chars};

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
        s: [Chars; n],
    }

    let mut result = u32::MAX;
    for i in 0usize..(1 << n) {
        let b = (0usize..m).all(|j| {
            (0usize..n)
                .filter(|&k| i.bit_test(k))
                .any(|k| s[k][j] == 'o')
        });
        if b {
            result = result.min(i.count_ones());
        }
    }
    println!("{result}");
}
