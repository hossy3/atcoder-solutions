use itertools::Itertools;
use proconio::input;

fn f(c: usize, w: &[usize], mut max: usize) -> usize {
    if w.len() == 0 {
        return 0;
    }

    let n = w.len();
    max = max.min(n);

    let mut sum = 0;
    for i in 0..max {
        sum += w[i];
        if sum > c {
            max = i;
            break;
        }
    }

    let mut result = usize::MAX;
    for i in (1..=max).rev() {
        if result <= n / i {
            break;
        }
        for v in (0..n).combinations(i) {
            if v.iter().map(|&i| w[i]).sum::<usize>() > c {
                continue;
            }
            let mut w0 = vec![];
            for j in 0..n {
                if !v.contains(&j) {
                    w0.push(w[j]);
                }
            }
            result = result.min(f(c, &w0, max));
        }
    }

    result += 1;
    result
}

fn main() {
    input! {
        n: usize,
        c: usize,
        mut w: [usize; n],
    }

    w.sort_unstable();

    let result = f(c, &w, n);
    println!("{result}");
}
