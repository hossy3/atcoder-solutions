use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    for i in 1..=n {
        if i % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{i}");
        }
    }
}
