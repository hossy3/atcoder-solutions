use proconio::input;

fn main() {
    input! {
        mut h: usize,
    }
    let mut k = 1usize; // 何体いるか
    let mut result = 0;
    while h > 1 {
        result += k;
        h /= 2;
        k *= 2;
    }
    result += k;
    println!("{result}");
}
