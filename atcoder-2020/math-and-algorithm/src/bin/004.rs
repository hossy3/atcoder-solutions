use proconio::input;

fn main() {
    input! {
        a: [usize; 3],
    }
    let result = a[1..].iter().fold(a[0], |acc, &x| acc * x);
    println!("{}", result);
}
