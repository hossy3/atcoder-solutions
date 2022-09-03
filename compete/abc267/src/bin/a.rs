use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let n = match &*s {
        "Monday" => 5,
        "Tuesday" => 4,
        "Wednesday" => 3,
        "Thursday" => 2,
        "Friday" => 1,
        _ => 0,
    };
    println!("{}", n);
}
