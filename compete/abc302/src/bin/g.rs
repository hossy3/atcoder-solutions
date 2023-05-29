use std::collections::{HashMap, HashSet};

use proconio::input;

fn f(a: &mut Vec<usize>) -> usize {
    let n = a.len();
    let mut a0 = a.clone();
    a0.sort();
    let a0 = a0;

    let mut map = HashMap::new();
    for i in 0..n {
        let x = a[i];
        let x0 = a0[i];
        if x != x0 {
            map.entry((x0, x)).or_insert(HashSet::new()).insert(i);
        }
    }

    let mut result = 0;
    while map.len() > 0 {
        let mut r = None;
        for (&(x, x0), i) in &map {
            if let Some(j) = map.get(&(x0, x)) {
                r = Some((*i.iter().next().unwrap(), *j.iter().next().unwrap(), x, x0));
                break;
            }
        }
        if let Some((i, j, x, x0)) = r {
            a.swap(i, j);
            map.get_mut(&(x, x0)).unwrap().remove(&i);
            if map.get(&(x, x0)).unwrap().len() == 0 {
                map.remove(&(x, x0));
            }
            map.get_mut(&(x0, x)).unwrap().remove(&j);
            if map.get(&(x0, x)).unwrap().len() == 0 {
                map.remove(&(x0, x));
            }
            result += 1;
            continue;
        }

        let mut r = None;
        let mut it = map.iter();
        let (&(x, x0), i) = it.next().unwrap();
        while let Some((&(y, y0), j)) = it.next() {
            if x == y0 || x0 == y {
                r = Some((
                    *i.iter().next().unwrap(),
                    *j.iter().next().unwrap(),
                    x,
                    x0,
                    y,
                    y0,
                ));
                break;
            }
        }
        if let Some((i, j, x, x0, y, y0)) = r {
            a.swap(i, j);
            map.get_mut(&(x, x0)).unwrap().remove(&i);
            if map.get(&(x, x0)).unwrap().len() == 0 {
                map.remove(&(x, x0));
            }
            map.get_mut(&(y, y0)).unwrap().remove(&j);
            if map.get(&(y, y0)).unwrap().len() == 0 {
                map.remove(&(y, y0));
            }
            if a[i] != a0[i] {
                map.entry((x, y0)).or_insert(HashSet::new()).insert(i);
            }
            if a[j] != a0[j] {
                map.entry((y, x0)).or_insert(HashSet::new()).insert(j);
            }
            result += 1;
            continue;
        }

        panic!();
    }

    result
}

#[test]
fn test_func() {
    assert_eq!(f(&mut vec![3, 4, 1, 1, 2, 4]), 3);
}

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    let result = f(&mut a);
    println!("{}", result);
}
