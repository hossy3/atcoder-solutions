use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        mut a: [usize; n],
    }
    a.sort();
    let yes = a.binary_search(&x).is_ok();
    println!("{}", if yes { "Yes" } else { "No" });
}
