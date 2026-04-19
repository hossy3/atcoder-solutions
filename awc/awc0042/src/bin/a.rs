use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    }

    let result = ab.iter().map(|&(a, b)| (a + b).div_ceil(k)).sum::<usize>();
    println!("{result}");
}
