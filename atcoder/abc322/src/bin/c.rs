use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }
    for i in 1..=n {
        let x = a.binary_search(&i);
        let result = if x.is_ok() { 0 } else { a[x.unwrap_err()] - i };
        println!("{result}");
    }
}
