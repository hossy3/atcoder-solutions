use proconio::input;

fn main() {
    input! {
        n: usize,
        tp: [(String, usize); n],
    }

    let result = tp
        .iter()
        .map(|(t, p)| if t.as_str() == "normal" { *p } else { *p / 2 })
        .sum::<usize>();
    println!("{result}");
}
