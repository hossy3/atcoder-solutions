use proconio::input;

trait BitTest {
    fn bit_test(&self, i: usize) -> bool;
}

impl BitTest for usize {
    fn bit_test(&self, i: usize) -> bool {
        self & (1 << i) != 0
    }
}

fn f(mask: usize, a: &[Vec<usize>]) -> usize {
    let h = a.len();
    let w = a[0].len();
    let mut result = 0;
    for i in 0..h {
        for j in 0..w {
            if !mask.bit_test(i * w + j) {
                result ^= a[i][j];
            }
        }
    }
    result
}

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let mut stack = vec![];
    stack.push((0, 0)); // 0番目の場所、マスクなし
    let mut result = f(0, &a);
    while let Some((i, mask)) = stack.pop() {
        if i + 1 < h * w {
            stack.push((i + 1, mask));
        }
        if mask.bit_test(i) {
            continue;
        }
        let i0 = i % w;
        if i0 < w - 1 && !mask.bit_test(i + 1) {
            // 横にドミノを置く
            let mask = mask | 1 << i | 1 << (i + 1);
            result = result.max(f(mask, &a));
            if i + 2 < h * w {
                stack.push((i + 2, mask));
            }
        }

        if i + w < h * w && !mask.bit_test(i + w) {
            // 縦にドミノを置く
            let mask = mask | 1 << i | 1 << (i + w);
            result = result.max(f(mask, &a));
            if i + 1 < h * w {
                stack.push((i + 1, mask));
            }
        }
    }

    println!("{result}");
}
