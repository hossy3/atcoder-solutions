use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: [usize; n],
    }
    x.sort();
    let result = x.windows(2).map(|v| v[1] - v[0]).max().unwrap_or(0);
    println!("{result}");
}
