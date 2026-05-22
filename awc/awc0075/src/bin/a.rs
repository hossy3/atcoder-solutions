use proconio::input;

fn main() {
    input! {
        n: usize,
        r: [usize; n],
    }
    let result = r
        .windows(3)
        .filter(|w| (w[0] < w[1] && w[1] > w[2]) || (w[0] > w[1] && w[1] < w[2]))
        .count();
    println!("{result}");
}
