use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [isize; n],
    }

    let mut sum = vec![0isize; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + a[i];
    }

    let mut cand = vec![];
    for l in 0..n {
        for r in (l + 1)..=(n.min(l + k)) {
            cand.push(sum[r] - sum[l]);
        }
    }
    cand.sort();

    let result = cand[(cand.len() - m)..].iter().sum::<isize>();
    println!("{result}");
}
