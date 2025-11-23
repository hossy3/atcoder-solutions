use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }
    let x: usize = xy.iter().map(|&(x, _)| x).sum();
    let y: usize = xy.iter().map(|&(_, y)| y).sum();
    let result = if x > y {
        "Takahashi"
    } else if x < y {
        "Aoki"
    } else {
        "Draw"
    };
    println!("{result}");
}
