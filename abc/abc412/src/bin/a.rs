use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let result = ab.iter().filter(|(a, b)| a < b).count();
    println!("{result}");
}
