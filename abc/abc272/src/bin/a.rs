use proconio::input;

fn main() {
    input! {
        a: [usize],
    }
    let sum: usize = a.iter().sum();
    println!("{}", sum);
}
