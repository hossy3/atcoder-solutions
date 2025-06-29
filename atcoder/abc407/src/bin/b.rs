use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }

    let mut count = 0;
    for i in 1..=6 {
        for j in 1..=6 {
            if i + j >= x || i.abs_diff(j) >= y {
                count += 1;
            }
        }
    }
    let result = count as f64 / 36.0;
    println!("{result}");
}
