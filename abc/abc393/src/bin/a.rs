use proconio::input;

fn main() {
    input! {
        s: [String; 2],
    }
    let result = if s[0].as_str() == "fine" {
        if s[1].as_str() == "fine" {
            4
        } else {
            3
        }
    } else {
        if s[1].as_str() == "fine" {
            2
        } else {
            1
        }
    };
    println!("{result}");
}
