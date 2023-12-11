use proconio::input;

fn main() {
    input! {
        a: [usize; 4],
    }
    let result = a.iter().min().unwrap();
    println!("{result}");
}
