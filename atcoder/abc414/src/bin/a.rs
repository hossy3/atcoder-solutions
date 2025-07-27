use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        xy: [(usize, usize); n],
    }
    let result = xy.iter().filter(|&&(x, y)| x <= l && y >= r).count();
    println!("{result}");
}
