use proconio::input;

trait BitTest {
    fn bit_test(&self, i: usize) -> bool;
}

impl BitTest for usize {
    fn bit_test(&self, i: usize) -> bool {
        self & (1 << i) != 0
    }
}

fn f(xy: &[(isize, isize)]) -> usize {
    let n = xy.len();
    let mut state = vec![vec![usize::MAX; 1 << n]; n];
    let mut stack = vec![];
    stack.push((0usize, 0usize, 1usize));
    let mut result = usize::MAX;

    let dist2 = |(x0, y0): (isize, isize), (x1, y1): (isize, isize)| {
        x1.abs_diff(x0).pow(2) + y1.abs_diff(y0).pow(2)
    };

    while let Some((i, score, bits)) = stack.pop() {
        // eprintln!("{}, {}, {:?}", i, score, &set);
        if score >= state[i][bits] {
            continue;
        }
        state[i][bits] = score;
        if bits == (1 << n) - 1 {
            result = result.min(score + dist2(xy[0], xy[i]));
        }

        for j in 0..n {
            if i == j || bits.bit_test(j) {
                continue;
            }
            let bits = bits | 1 << j;
            let score = score + dist2(xy[i], xy[j]);
            stack.push((j, score, bits));
        }
    }

    result
}

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    }
    let result = f(&xy);
    println!("{result}");
}
