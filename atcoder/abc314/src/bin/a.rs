use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    const S: &str = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679";
    let result = &S[..(n + 2)];
    println!("{result}");
}
