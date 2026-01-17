use proconio::input;

fn main() {
    input! {
        p: usize,
        q: usize,
        x: usize,
        y: usize,
    }

    let yes = p <= x && x < p + 100 && q <= y && y < q + 100;
    println!("{}", if yes { "Yes" } else { "No" });
}
