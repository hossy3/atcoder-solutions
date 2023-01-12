use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        ab: [(usize, char); n],
    }

    let result = ab
        .iter()
        .map(|&(a, b)| if b == 'E' { l - a } else { a })
        .max()
        .unwrap();
    println!("{}", result);
}
