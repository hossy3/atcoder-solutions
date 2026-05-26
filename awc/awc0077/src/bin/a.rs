use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        m: usize,
        pk: [(usize, usize); n],
    }

    let result = pk
        .iter()
        .filter(|&&(p, k)| l <= p && p <= r && k <= m)
        .count();
    println!("{result}");
}
