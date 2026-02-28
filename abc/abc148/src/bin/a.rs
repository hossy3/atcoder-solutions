use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let a = [a, b];
    let result = if !a.contains(&1) {
        1
    } else if !a.contains(&2) {
        2
    } else {
        3
    };
    println!("{result}");
}
