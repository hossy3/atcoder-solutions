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
        k: usize,
        cp: [(usize, usize); n],
        uv: [(Usize1, Usize1); m],
    }

    let mut uv0 = vec![];
    for (u, v) in uv {
        uv0.push((1 << u) | (1 << v));
    }

    let mut result = 0usize;
    for i in 0..(1 << n) {
        if uv0.iter().any(|&uv0| i & uv0 == uv0) {
            continue; // U, V の両方を選ぶことはできない
        }
        let cost = (0..n)
            .filter(|&k| i.bit_test(k))
            .map(|j| cp[j].0)
            .sum::<usize>();
        if cost > k {
            continue; // コストが予算を超える
        }
        let p = (0..n)
            .filter(|&k| i.bit_test(k))
            .map(|j| cp[j].1)
            .sum::<usize>();
        result = result.max(p);
    }

    println!("{result}");
}
