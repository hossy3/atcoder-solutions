use proconio::input;

fn main() {
    input! {
        x: f64,
    }
    let x = x.round();
    println!("{}", x);
}
