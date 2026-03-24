use proconio::input;

fn main() {
    input! {
        n: usize,
        r: isize,
        xypq: [(isize, isize, isize, isize); n],
    }
    let result = xypq
        .iter()
        .filter(|(x, y, p, q)| (x - p).pow(2) + (y - q).pow(2) <= r.pow(2))
        .count();
    println!("{result}");
}
