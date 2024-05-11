use proconio::input;

fn main() {
    input! {
        _: usize,
        x: usize,
        y: usize,
        z: usize,
    }
    let min = x.min(y);
    let max = x.max(y);
    let yes = min < z && z < max;
    println!("{}", if yes { "Yes" } else { "No" });
}
