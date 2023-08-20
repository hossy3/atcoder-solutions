use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
    }
    let mut x = vec![a, b, c, d, e];
    x.sort();
    let yes = (x[0] == x[1] && x[1] == x[2] && x[2] != x[3] && x[3] == x[4])
        || (x[0] == x[1] && x[1] != x[2] && x[2] == x[3] && x[3] == x[4]);
    println!("{}", if yes { "Yes" } else { "No" });
}
