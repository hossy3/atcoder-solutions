use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        c: usize,
        d: usize,
        w: [usize; n],
    }

    let result = (0..n)
        .map(|i| if w[i] < t { 0 } else { d.min(c) })
        .sum::<usize>();
    println!("{result}");
}
