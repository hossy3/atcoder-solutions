use proconio::input;

// 最大作れる数を返す
fn f(q: &[usize], a: &[usize]) -> usize {
    let mut result = usize::MAX;
    for (&q, &a) in std::iter::zip(q, a) {
        if a > 0 {
            result = result.min(q / a);
        }
    }
    result
}

fn main() {
    input! {
        n: usize,
        q: [usize; n],
        a: [usize; n],
        b: [usize; n],
    }

    let mut result = 0;
    let a_max = f(&q, &a);
    for a0 in 0..=a_max {
        let q: Vec<usize> = (0..n).map(|i| q[i] - a0 * a[i]).collect();
        let b0 = f(&q, &b);
        result = result.max(a0 + b0);
    }
    println!("{result}");
}
