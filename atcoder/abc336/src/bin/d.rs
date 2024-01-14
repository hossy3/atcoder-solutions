use std::iter;

use proconio::input;

fn f(a: &[usize]) -> usize {
    let n = a.len();

    let mut a0 = vec![0usize; n];
    a0[0] = 1;
    for i in 0..(n - 1) {
        a0[i + 1] = a[i + 1].min(a0[i] + 1);
    }

    let mut a1 = vec![0usize; n];
    a1[n - 1] = 1;
    for i in (1..n).rev() {
        a1[i - 1] = a[i - 1].min(a1[i] + 1);
    }

    iter::zip(a0, a1).map(|(x0, x1)| x0.min(x1)).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(f(&[1, 2, 3, 4]), 2);
        assert_eq!(f(&[5, 4, 3, 2]), 2);
        assert_eq!(f(&[1, 2, 3, 4, 5]), 3);
        assert_eq!(f(&[5, 4, 3, 2, 1]), 3);
    }
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let result = f(&a);
    println!("{result}");
}
