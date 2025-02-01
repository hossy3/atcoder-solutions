use proconio::input;

fn main() {
    input! {
        d: String,
    }
    let result = match d.as_str() {
        "N" => "S",
        "E" => "W",
        "W" => "E",
        "S" => "N",
        "NE" => "SW",
        "NW" => "SE",
        "SE" => "NW",
        "SW" => "NE",
        _ => unreachable!(),
    };
    println!("{result}");
}
