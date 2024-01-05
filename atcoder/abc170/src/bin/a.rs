use proconio::input;

fn main() {
    input! {
        x: [usize; 5],
    }
    let result = (0..=4).find(|&i| x[i] == 0).unwrap() + 1;
    println!("{result}");
}
