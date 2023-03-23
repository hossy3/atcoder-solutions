use proconio::input;

fn main() {
    input! {
        a: [usize; 3],
    }
    let result: usize = a.iter().sum();
    println!("{}", result);
}
