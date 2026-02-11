use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let result = if a < b {
        a.to_string().repeat(b)
    } else {
        b.to_string().repeat(a)
    };
    println!("{result}");
}
