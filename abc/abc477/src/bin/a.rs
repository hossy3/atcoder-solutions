use proconio::input;

fn main() {
    input! {
        c: char,
    }
    let result = match c {
        'B' => 'Y',
        'Y' => 'R',
        'R' => 'B',
        _ => unreachable!(),
    };
    println!("{result}");
}
