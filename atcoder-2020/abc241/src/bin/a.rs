use proconio::input;

fn main() {
    input! {
        a: [usize; 10],
    }
    let result = a[a[a[0]]];
    println!("{}", result);
}
