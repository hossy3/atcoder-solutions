use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }
    let sum = a.iter().map(|a| a.iter().sum::<usize>()).sum::<usize>();
    let min = *a.iter().map(|a| a.iter().min().unwrap()).min().unwrap();
    let result = sum - min * h * w;
    println!("{result}");
}
