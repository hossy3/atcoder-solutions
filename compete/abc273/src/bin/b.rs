use proconio::input;

fn main() {
    input! {
        x: usize,
        k: u32,
    }
    let mut result = x;
    for i in 0..k {
        let y = (result / 10usize.pow(i)) % 10;
        let z = 10usize.pow(i + 1);
        if y < 5 {
            result = (result / z) * z;
        } else {
            result = (result / z + 1) * z;
        }
    }
    println!("{}", result);
}
