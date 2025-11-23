use proconio::input;

fn main() {
    input! {
        x: i64,
    }
    let y = x.div_euclid(10);
    let result = if y * 10 == x { y } else { y + 1 };
    println!("{result}");
}
