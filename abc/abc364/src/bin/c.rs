use proconio::input;

fn f(a: &[usize], x: usize) -> usize {
    let mut sum = 0;
    let mut count = 0;
    for a in a.iter().rev() {
        count += 1;
        sum += a;
        if sum > x {
            break;
        }
    }
    count
}

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        mut a: [usize; n],
        mut b: [usize; n],
    }

    a.sort();
    b.sort();

    let count_a = f(&a, x);
    let count_b = f(&b, y);
    let result = count_a.min(count_b);
    println!("{result}");
}
