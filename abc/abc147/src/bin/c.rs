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
    }

    let mut v = vec![];
    for _ in 0..n {
        input! {
            a: usize,
            xy: [(Usize1, u8); a],
        }
        v.push(xy);
    }

    let mut result = 0;
    'outer: for i in 0..(1usize << n) {
        for j in 0..n {
            if !i.bit_test(j) {
                continue; // 不親切な人の発言は無視する
            }
            for &(x, y) in &v[j] {
                let b = i.bit_test(x);
                if (b && y == 0) || (!b && y == 1) {
                    continue 'outer; // 矛盾があった
                }
            }
        }
        result = result.max(i.count_ones());
    }

    println!("{result}");
}
