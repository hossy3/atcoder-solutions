use proconio::input;

fn f(n: usize, k: usize, a: &[usize]) -> usize {
    let mut sum = 0;
    for i in 0..n {
        sum += a[i];
        if a[i] == a[i + 1] {
            continue;
        }
        let result = (sum + (n - i) * a[i + 1]) / k;
        if result >= a[i + 1] {
            continue;
        }
        return sum / (i + k - n);
    }

    sum += a[n];
    return sum / k;
}

#[test]
fn test_func() {
    assert_eq!(f(3, 3, &[0, 2, 3, 4]), 2);
    assert_eq!(f(3, 3, &[0, 2, 3000, 4000]), 2);

    assert_eq!(f(4, 2, &[0, 1, 1, 3, 4]), 4);
    assert_eq!(f(4, 2, &[0, 1, 1, 3000, 4000]), 3002);

    assert_eq!(f(4, 3, &[0, 1, 1, 3, 4]), 2);
    assert_eq!(f(4, 3, &[0, 1, 1, 3000, 4000]), 2);
}

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    a.push(0);
    a.sort();
    let result = f(n, k, &a);
    println!("{}", result);
}
