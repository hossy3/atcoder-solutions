use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut a_cum = vec![0usize; n + 1];
    for i in 0..n {
        a_cum[i + 1] = a_cum[i] + a[i];
    }

    let mut b_cum = vec![0usize; n + 1];
    for i in 0..n {
        b_cum[i + 1] = b_cum[i] + b[i];
    }

    let mut result = 0usize;
    for l in 0..n {
        let x = b_cum[l] + k;
        let r = b_cum.partition_point(|&x0| x0 <= x);
        result = result.max(a_cum[r - 1].saturating_sub(a_cum[l]));
    }
    println!("{result}");
}
