use proconio::input;

fn main() {
    input! {
        n: String,
    }
    let result = usize::from_str_radix(n.as_str(), 2).unwrap();
    println!("{}", result);
}
