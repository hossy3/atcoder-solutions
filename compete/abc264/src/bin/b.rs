use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
    }
    let r = r.min(16 - r);
    let c = c.min(16 - c);
    let black = r.min(c) % 2 == 1;
    println!("{}", if black { "black" } else { "white" });
}
