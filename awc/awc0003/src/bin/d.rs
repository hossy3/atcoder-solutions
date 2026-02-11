use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        a: [usize; n],
    }

    let mut cum = vec![0usize; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + a[i];
    }

    let mut result = 0usize;
    for i in 0..=(n - k) {
        let j = cum.partition_point(|&x| x < cum[i] + m).max(i + k);
        result += n + 1 - j;
    }
    println!("{result}");
}
