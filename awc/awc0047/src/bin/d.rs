use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let sum = a.iter().sum::<usize>();
    if sum % n != 0 {
        println!("-1"); // 割り切れない
        return;
    }

    let target = sum / n;
    let mut diff = 0isize;
    let mut result = 0usize;
    for a in a {
        result += diff.abs() as usize;
        diff += a as isize - target as isize;
    }
    println!("{result}");
}
