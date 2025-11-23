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
        h: usize,
        w: usize,
        k: usize,
        c: [Chars; h],
    }

    let mut result = 0;
    for h0 in 0..(1usize << h) {
        for w0 in 0..(1usize << w) {
            let mut count = 0;
            for i in 0..h {
                for j in 0..w {
                    if c[i][j] == '#' && !h0.bit_test(i) && !w0.bit_test(j) {
                        count += 1;
                    }
                }
            }
            if count == k {
                result += 1;
            }
        }
    }
    println!("{result}");
}
