use proconio::input;

fn main() {
    input! {
        x: f64,
    }

    let y = x - x.floor();
    let x = x.floor();
    if y < 0.3 {
        println!("{}-", x);
    } else if y < 0.7 {
        println!("{}", x);
    } else {
        println!("{}+", x);
    }
}
