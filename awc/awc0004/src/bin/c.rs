use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [isize; n],
    }
    let max = h.iter().max().unwrap();
    let min = h.iter().min().unwrap();
    let result = if max * min > 0 {
        max.abs().max(min.abs()) * 2
    } else {
        (max - min) * 2
    };
    println!("{result}");
}
