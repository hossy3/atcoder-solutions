use proconio::input;

fn main() {
    input! {
        x: usize,
    }
    let mut y = 1usize;
    for i in 1..=100 {
        y *= i;
        if y == x {
            println!("{i}");
            break;
        }
    }
}
