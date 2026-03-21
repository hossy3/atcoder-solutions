use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }
    for x in a {
        let (q, r) = (x / m, x % m);
        println!("{q} {r}");
    }
}
