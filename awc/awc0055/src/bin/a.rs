use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        d: [usize; n],
    }

    let result = (0..n)
        .filter(|&i| i % (k + 1) < k)
        .map(|i| d[i])
        .sum::<usize>();
    println!("{result}");
}
