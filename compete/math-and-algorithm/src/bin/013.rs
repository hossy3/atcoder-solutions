use proconio::input;

fn prime_division(mut n: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let mut k = 2;
    while k * k <= n {
        let mut count = 0;
        while n % k == 0 {
            count += 1;
            n /= k;
        }
        if count > 0 {
            result.push((k, count));
        }
        k += if k == 2 { 1 } else { 2 };
    }
    if n > 1 {
        result.push((n, 1));
    }
    result
}

#[test]
fn test_prime_division() {
    assert_eq!(prime_division(2), [(2, 1)]);
    assert_eq!(prime_division(3), [(3, 1)]);
    assert_eq!(prime_division(4), [(2, 2)]);
    assert_eq!(prime_division(5), [(5, 1)]);
    assert_eq!(prime_division(6), [(2, 1), (3, 1)]);
    assert_eq!(prime_division(7), [(7, 1)]);
    assert_eq!(prime_division(8), [(2, 3)]);
    assert_eq!(prime_division(9), [(3, 2)]);
}

fn collect_divisor(n: usize) -> Vec<usize> {
    let a = prime_division(n);
    let mut v = vec![1];
    for &(k, count) in &a {
        let prev = v.clone();
        for i in 1..=count {
            let k = k.pow(i as u32);
            for &x in &prev {
                v.push(k * x);
            }   
        }
    }

    v.sort();
    v
}

#[test]
fn test_collect_divisor() {
    assert_eq!(collect_divisor(2), [1, 2]);
    assert_eq!(collect_divisor(3), [1, 3]);
    assert_eq!(collect_divisor(4), [1, 2, 4]);
    assert_eq!(collect_divisor(5), [1, 5]);
    assert_eq!(collect_divisor(6), [1, 2, 3, 6]);
    assert_eq!(collect_divisor(7), [1, 7]);
    assert_eq!(collect_divisor(8), [1, 2, 4, 8]);
    assert_eq!(collect_divisor(9), [1, 3, 9]);
}

fn main() {
    input! {
        n: usize,
    }
    let a = collect_divisor(n);
    for x in a {
        println!("{}", x);
    }
}
