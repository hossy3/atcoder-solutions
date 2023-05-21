use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;

fn f(k: usize, a: &[usize]) -> Vec<usize> {
    let mut v = Vec::with_capacity(k);
    let mut set0 = BTreeSet::new(); // (1, 3) means 1..=3
    set0.insert((0usize, 0usize));
    'outer: for &x in a {
        // skip test
        {
            let mut k0 = 0;
            let mut r1 = 0;
            for &(l, r) in &set0 {
                if l > 0 {
                    k0 += l - r1 - 1;
                }
                if k0 >= k * 2 {
                    break 'outer;
                }
                if r >= x {
                    break;
                }
                r1 = r;
            }
        }

        let prev = set0.clone();
        for (l, r) in prev {
            set0.insert((l + x, r + x));
        }

        // merge
        let mut set1 = BTreeSet::new();
        let mut it = set0.iter();
        let (mut l0, mut r0) = (0, 0);
        if let Some(x0) = it.next() {
            l0 = x0.0;
            r0 = x0.1;
        }
        while let Some(x1) = it.next() {
            let &(l1, r1) = x1;
            if r0 + 1 < l1 {
                set1.insert((l0, r0));
                l0 = l1;
                r0 = r1;
            } else {
                r0 = r0.max(r1);
            }
        }
        set1.insert((l0, r0));
        set0 = set1;
    }

    let mut it = set0.iter();
    let mut r0 = 0;
    if let Some(x0) = it.next() {
        r0 = x0.1;
    }
    while let Some(x1) = it.next() {
        let &(l1, r1) = x1;
        for i in (r0 + 1)..l1 {
            v.push(i);
            if v.len() == k {
                return v;
            }
        }
        r0 = r1;
    }
    let r1 = r0 + 1 + k - v.len();
    for i in (r0 + 1)..r1 {
        v.push(i);
    }
    v
}

#[test]
fn test_func() {
    assert_eq!(f(3, &[1, 2, 5]), vec![4, 9, 10]);
    assert_eq!(f(3, &[2, 5]), vec![1, 3, 4]);
    assert_eq!(f(1, &[100]), vec![1]);
    assert_eq!(f(2, &[100]), vec![1, 2]);
    assert_eq!(f(5, &[4]), vec![1, 2, 3, 5, 6]);
    assert_eq!(f(3, &[2, 2, 2]), vec![1, 3, 5]);
    assert_eq!(f(4, &[2, 2, 2]), vec![1, 3, 5, 7]);
    assert_eq!(f(5, &[2, 2, 2]), vec![1, 3, 5, 7, 8]);
}

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }
    a.sort();
    let result = f(k, &a).iter().join(" ");
    println!("{}", result);
}
