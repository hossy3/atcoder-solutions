use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        h: [usize; n],
        c: [usize; n],
    }
    let result = (0..n)
        .map(|i| if h[i] <= t { c[i] } else { 0 })
        .sum::<usize>();
    println!("{result}");
}
