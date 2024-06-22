use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let count_lower = s.iter().filter(|c| c.is_ascii_lowercase()).count();
    let count_upper = s.iter().filter(|c| c.is_ascii_uppercase()).count();
    if count_lower > count_upper {
        let result = s.iter().map(|c| c.to_ascii_lowercase()).join("");
        println!("{result}");
    } else {
        let result = s.iter().map(|c| c.to_ascii_uppercase()).join("");
        println!("{result}");
    }
}
