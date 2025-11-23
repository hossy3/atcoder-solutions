use proconio::input;

fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize,
        c: String,
    }
    let result = match c.as_str() {
        "Red" => g.min(b),
        "Green" => b.min(r),
        "Blue" => r.min(g),
        _ => unreachable!(),
    };
    println!("{result}");
}
