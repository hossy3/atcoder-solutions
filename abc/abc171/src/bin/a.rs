use proconio::input;

fn main() {
    input! {
        a: char,
    }
    let result = match a {
        'a'..='z' => 'a',
        'A'..='Z' => 'A',
        _ => unreachable!(),
    };
    println!("{result}");
}
