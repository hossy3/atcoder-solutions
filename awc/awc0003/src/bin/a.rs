use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    }
    let result = ab.iter().filter(|&&(a, b)| a * b >= k).count();
    println!("{result}");
}
