use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let yes = a
        .iter()
        .all(|&x| (x % 2 == 1) || (x % 3 == 0) || (x % 5 == 0));
    println!("{}", if yes { "APPROVED" } else { "DENIED" });
}
