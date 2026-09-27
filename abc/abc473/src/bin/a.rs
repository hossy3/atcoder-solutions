use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let result = a[(a.len() / 2)..].iter().sum::<usize>();
    println!("{result}");
}
