use std::collections::HashSet;

use proconio::input;

trait BitTest {
    fn bit_test(&self, i: usize) -> bool;
}

impl BitTest for usize {
    fn bit_test(&self, i: usize) -> bool {
        self & (1 << i) != 0
    }
}

fn f(cur: usize, a: &[usize], set: &mut HashSet<usize>) {
    if a.len() == 0 {
        set.insert(cur);
        return;
    }

    // 全探索
    let n = a.len() - 1;
    for i in 0..(1 << n) {
        let mut x = a[0];
        let mut v = vec![];
        for j in 0..n {
            if i.bit_test(j) {
                x += a[j + 1];
            } else {
                v.push(a[j + 1]);
            }
        }
        f(cur ^ x, &v, set);
    }
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut set = HashSet::new();
    f(0, &a, &mut set);
    let result = set.len();
    println!("{result}");
}
