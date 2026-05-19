use proconio::{input, marker::Usize1};

fn f(cs: &[Vec<usize>], w: &[usize], i: usize) -> (usize, usize) {
    if cs[i].len() <= 0 {
        return (0, w[i]);
    }

    let mut score = 0;
    let mut sum = 0;
    let mut max = usize::MIN;
    let mut min = usize::MAX;
    for &i in &cs[i] {
        let (score0, sum0) = f(cs, w, i);
        score = score.max(score0);
        sum += sum0;
        min = min.min(sum0);
        max = max.max(sum0);
    }
    score = score.max(max - min);

    (score, sum)
}

fn main() {
    input! {
        n: usize,
        p: [Usize1; n - 1],
        w: [usize; n],
    }
    let mut cs = vec![vec![]; n];
    for (i, &p) in p.iter().enumerate() {
        cs[p].push(i + 1);
    }
    let (result, _) = f(&cs, &w, 0);
    println!("{result}");
}
