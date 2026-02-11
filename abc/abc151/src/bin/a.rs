use proconio::input;

fn main() {
    input! {
        c: char,
    }
    let result = (c as u8 + 1) as char;
    println!("{result}");
}
