use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        s: [usize; n],
    }
    let result: usize = s.iter().map(|&y| if y <= x { y } else { 0 }).sum();
    println!("{result}");
}
