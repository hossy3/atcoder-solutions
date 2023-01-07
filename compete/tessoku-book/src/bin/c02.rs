use proconio::input;

fn main() {
    input! {
        mut a: [usize],
    }
    a.sort();
    let result = a[a.len() - 1] + a[a.len() - 2];
    println!("{}", result);
}
