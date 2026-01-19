use proconio::input;

fn main() {
    input! {
        k: usize,
        a: usize,
        b: usize,
    }
    let x = (b / k) * k;
    let yes = a <= x && x <= b;
    println!("{}", if yes { "OK" } else { "NG" });
}
