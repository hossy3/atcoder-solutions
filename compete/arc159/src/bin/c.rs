use itertools::Itertools;
use proconio::input;

fn swap_arrays(x: usize, y: usize, n: usize) -> (Vec<usize>, Vec<usize>) {
    let mut v = (Vec::with_capacity(n), Vec::with_capacity(n));
    for i in 1..=(n - 2) {
        v.0.push(i);
        v.1.push(n + 1 - i);
    }
    if x < y {
        v.0.insert(x, n);
        v.0.insert(y, n - 1);
        v.1.insert(x, 2);
        v.1.insert(y, 1);
    } else {
        v.0.insert(y, n - 1);
        v.0.insert(x, n);
        v.1.insert(y, 1);
        v.1.insert(x, 2);
    }
    v
}

#[test]
fn test_func() {
    assert_eq!(swap_arrays(0, 1, 4), (vec![4, 3, 1, 2], vec![2, 1, 4, 3]));
    assert_eq!(swap_arrays(1, 0, 4), (vec![3, 4, 1, 2], vec![1, 2, 4, 3]));
    assert_eq!(swap_arrays(1, 2, 4), (vec![1, 4, 3, 2], vec![4, 2, 1, 3]));
    assert_eq!(swap_arrays(2, 1, 4), (vec![1, 3, 4, 2], vec![4, 1, 2, 3]));
}

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut offset = false;
    let mut sum: usize = a.iter().sum();
    if sum % n > 0 {
        for i in 0..n {
            a[i] += i + 1;
        }
        sum = a.iter().sum();
        offset = true;
    }
    let yes = sum % n == 0;
    println!("{}", if yes { "Yes" } else { "No" });
    if !yes {
        return;
    }

    let ave = sum / n;
    let mut v0 = Vec::new(); // < ave
    let mut v1 = Vec::new(); // > ave
    for (i, &x) in a.iter().enumerate() {
        if x < ave {
            v0.append(&mut vec![i; ave - x]);
        } else if x > ave {
            v1.append(&mut vec![i; x - ave]);
        }
    }

    if offset {
        println!("{}", v0.len() * 2 + 1);
        println!("{}", (1..=n).into_iter().join(" "));
    } else {
        println!("{}", v0.len() * 2);
    }

    for (&x, &y) in v0.iter().zip(v1.iter()) {
        let v = swap_arrays(x, y, n);
        println!("{}", v.0.iter().join(" "));
        println!("{}", v.1.iter().join(" "));
    }
}
