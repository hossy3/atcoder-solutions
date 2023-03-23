use num_integer::Roots;
use proconio::input;

fn is_prime(n: usize) -> bool {
    if n == 2 || n == 3 {
        return true;
    }

    let max = n.sqrt();
    for i in 2..=max {
        if n % i == 0 {
            return false;
        }
    }

    true
}

#[test]
fn test() {
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(4), false);
    assert_eq!(is_prime(5), true);
    assert_eq!(is_prime(6), false);
    assert_eq!(is_prime(7), true);
}

fn main() {
    input! {
        n: usize,
    }
    let yes = is_prime(n);
    println!("{}", if yes { "Yes" } else { "No" });
}
