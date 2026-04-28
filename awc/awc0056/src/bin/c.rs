use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut cum = vec![0; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + a[i];
    }

    let mut result = 0;
    for i in 0..n {
        let j = cum.partition_point(|&x| x <= cum[i] + k) - 1;
        result = result.max(j - i);
    }
    println!("{result}");
}
