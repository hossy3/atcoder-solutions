use itertools::Itertools;
use proconio::input;

fn f(x: usize, y: usize, a: &Vec<usize>) -> usize {
    let mut a = a.clone();
    a.push(x); // guard
    a.insert(0, x); // guard

    let n = (0..a.len())
        .filter(|&i| a[i] >= x || a[i] <= y)
        .tuple_windows()
        .map(|(i, j)| {
            if j > i + 1 {
                (j - i) * (j - i - 1) / 2
            } else {
                0
            }
        })
        .sum();
    n
}

#[test]
fn test_func() {
    assert_eq!(f(1, 1, &vec![1, 1, 1, 1, 1]), 0);
    assert_eq!(f(2, 1, &vec![1, 1, 1, 1, 1]), 0);
    assert_eq!(f(1, 0, &vec![1, 1, 1, 1, 1]), 0);
    assert_eq!(f(2, 0, &vec![1, 1, 1, 1, 1]), 15);
}

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        a: [usize; n],
    }

    let result = f(x + 1, y - 1, &a) + f(x, y, &a) - f(x + 1, y, &a) - f(x, y - 1, &a);
    println!("{}", result);
}
