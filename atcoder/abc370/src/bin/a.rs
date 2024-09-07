use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    }
    if l == r {
        println!("Invalid");
    } else {
        let yes = l == 1;
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
