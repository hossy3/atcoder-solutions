use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let result = match s.as_str() {
        "red" => "SSS",
        "blue" => "FFF",
        "green" => "MMM",
        _ => "Unknown",
    };
    println!("{result}");
}
