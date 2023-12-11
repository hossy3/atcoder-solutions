use proconio::input;

fn f(i: usize, base: usize) -> bool {
    if i == 0 {
        true
    } else if i % base == 7 {
        false
    } else {
        f(i / base, base)
    }
}

fn main() {
    input! {
        n: usize,
    }
    let result = (1..=n).filter(|&i| f(i, 10) && f(i, 8)).count();
    println!("{result}");
}
