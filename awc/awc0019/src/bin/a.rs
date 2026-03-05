use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [usize; n],
    }
    let result = s.iter().filter(|&&x| x >= k).count();
    println!("{result}");
}
