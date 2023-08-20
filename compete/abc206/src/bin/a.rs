use proconio::input;

fn main() {
    input! {
        n: f64,
    }
    let x = (n * 1.08).floor();
    let result = if x < 206.0 {
        "Yay!"
    } else if x == 206.0 {
        "so-so"
    } else {
        ":("
    };
    println!("{}", result);
}
