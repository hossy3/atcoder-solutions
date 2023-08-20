use proconio::input;

fn main() {
    input! {
        abc: usize,
    }
    let a = abc / 100;
    let b = (abc / 10) % 10;
    let c = abc % 10;
    let result = (a + b + c) * 111;
    println!("{}", result);
}
