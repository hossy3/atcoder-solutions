use itertools::Itertools;
use proconio::input;

trait BitTest {
    fn bit_test(&self, i: usize) -> bool;
}

impl BitTest for usize {
    fn bit_test(&self, i: usize) -> bool {
        self & (1 << i) != 0
    }
}

fn f(i: usize, n: usize) -> bool {
    let mut level = 0usize;
    for j in 0..n {
        if i.bit_test(j) {
            level += 1;
        } else {
            if level == 0 {
                return false;
            }
            level -= 1;
        }
    }

    level == 0
}

fn main() {
    input! {
        n: usize,
    }
    if n == 0 || n % 2 == 1 {
        return;
    }
    for i in 0..(1 << n) {
        if !f(i, n) {
            continue;
        }
        let result = (0..n)
            .rev()
            .map(|j| if i.bit_test(j) { ')' } else { '(' })
            .join("");
        println!("{result}");
    }
}
