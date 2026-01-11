use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut result = 0;
    for i in 0..n {
        let mut sum = 0;
        for j in i..n {
            sum += a[j];
            if (i..=j).all(|k| sum % a[k] > 0) {
                result += 1;
            }
        }
    }
    println!("{result}");
}
