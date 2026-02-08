use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut result = 0;
    for i in 1..=n {
        let mut i = i;
        let mut sum = 0;
        while i > 0 {
            sum += i % 10;
            i /= 10;
        }
        if sum == k {
            result += 1;
        }
    }
    println!("{result}");
}
