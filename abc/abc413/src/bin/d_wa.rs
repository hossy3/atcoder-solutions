use itertools::Itertools;
use proconio::input;

fn f(a: &[i64]) -> bool {
    let n = a.len();
    if n <= 2 {
        return true;
    }

    let a: Vec<_> = a.iter().cloned().sorted().collect();
    if a[0] == 1 && a[n - 1] == 1 {
        return true;
    }
    if a[0] == -1 && a[n - 1] == 1 {
        if n % 2 == 0 {
            if a[n / 2 - 1] == -1 && a[n / 2] == 1 {
                return true;
            }
        } else {
            if (a[n / 2 - 1] == -1 && a[n / 2] == 1) || (a[n / 2] == -1 && a[n / 2 + 1] == 1) {
                return true;
            }
        }
        return false;
    }

    let a: Vec<_> = a.iter().sorted_by_key(|&x| x.abs()).collect();
    // eprintln!("a: {:?}", &a);
    if a.windows(3).all(|v| v[0] * v[2] == v[1] * v[1]) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(f(&[1, 1, 1]), true);
        assert_eq!(f(&[-1, 1, 1]), true);
        assert_eq!(f(&[-1, -1, 1]), true);
        assert_eq!(f(&[-1, -1, -1]), true);

        assert_eq!(f(&[1, 1, 1, 1]), true);
        assert_eq!(f(&[-1, 1, 1, 1]), false);
        assert_eq!(f(&[-1, -1, 1, 1]), true);
        assert_eq!(f(&[-1, -1, -1, 1]), false);
        assert_eq!(f(&[-1, -1, -1, -1]), true);

        assert_eq!(f(&[1, 1, 1, 1, 1]), true);
        assert_eq!(f(&[-1, 1, 1, 1, 1]), false);
        assert_eq!(f(&[-1, -1, 1, 1, 1]), true);
        assert_eq!(f(&[-1, -1, -1, 1, 1]), true);
        assert_eq!(f(&[-1, -1, -1, -1, 1]), false);
        assert_eq!(f(&[-1, -1, -1, -1, -1]), true);
    }

    #[test]
    fn test_large() {
        assert_eq!(f(&[-10000, 10000]), true);
        assert_eq!(f(&[10000000, 100000000, 1000000000]), true);
        assert_eq!(f(&[-1000000, 10000000, -100000000, 1000000000]), true);
        assert_eq!(f(&[1000000, 10000000, -100000000, 1000000000]), false);
        assert_eq!(f(&[1000000000, -1000000000, 1000000000]), true);
    }

    #[test]
    fn test2() {
        assert_eq!(f(&[1, 8, 2, 4, 16]), true);
        assert_eq!(f(&[90000, 8100, -27000, 729, -300000, -2430, 1000000]), true);
        assert_eq!(f(&[90000, 8100, 27000, 729, 300000, 2430, 1000000]), true);
        assert_eq!(f(&[90000, 8100, -27000, 729, 300000, 2430, 1000000]), false);
    }

    #[test]
    fn test3() {
        assert_eq!(f(&[9, 12, 16]), true);
        assert_eq!(f(&[-9, 12, -16]), true);
        assert_eq!(f(&[9, -12, 16]), true);
        assert_eq!(f(&[-9, -12, -16]), true);
        assert_eq!(f(&[-9, 12, 16]), false);
    }
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            a: [i64; n]
        }
        let yes = f(&a);
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
