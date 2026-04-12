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
        l: [isize; n],
    }

    let mut result = 0isize;
    for bits in 0usize..(1 << n) {
        let mut cur = 0isize;
        let mut count = 0isize;
        for (i, &x) in l.iter().enumerate() {
            let prev = cur;
            cur = if bits.bit_test(i) { prev + x } else { prev - x };
            if (prev >= 0 && cur < 0) || (prev < 0 && cur >= 0) {
                count += 1;
            }
        }
        result = result.max(count);
    }

    println!("{result}");
}
