use std::collections::BTreeSet;

use proconio::input;

fn f(k: usize, a: &[usize]) -> bool {
    let n = a.len();

    // a[..=i] について左端から昇順が続いているか
    let mut v0 = vec![false; n];
    v0[0] = true;
    for i in 0..(n - 1) {
        if a[i] > a[i + 1] {
            break;
        }
        v0[i + 1] = true;
    }

    // a[i..] について右端まで昇順が続いているか
    let mut v1 = vec![false; n];
    v1[n - 1] = true;
    for i in (0..(n - 1)).rev() {
        if a[i] > a[i + 1] {
            break;
        }
        v1[i] = true;
    }

    // 区間の最小値と最大値
    let mut v_min = vec![];
    let mut v_max = vec![];
    let mut set = BTreeSet::new();
    for i in 0..k {
        set.insert((a[i], i));
    }
    let &(min, _) = set.first().unwrap();
    let &(max, _) = set.last().unwrap();
    v_min.push(min);
    v_max.push(max);
    for i in k..n {
        set.remove(&(a[i - k], i - k));
        set.insert((a[i], i));
        let &(min, _) = set.first().unwrap();
        let &(max, _) = set.last().unwrap();
        v_min.push(min);
        v_max.push(max);
    }

    // eprintln!("{:?}", &v0);
    // eprintln!("{:?}", &v1);
    // eprintln!("{:?}", &v_min);
    // eprintln!("{:?}", &v_max);

    // ひとつでも該当すれば良い
    for i in 0..(n - k + 1) {
        if i > 0 {
            if !v0[i - 1] {
                continue;
            }
            if a[i - 1] > v_min[i] {
                continue;
            }
        }
        if i + k < n {
            if !v1[i + k] {
                continue;
            }
            if a[i + k] < v_max[i] {
                continue;
            }
        }

        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert!(f(3, &[1, 2, 1, 2, 1]) == true);
        assert!(f(4, &[1, 2, 1, 2, 1]) == true);
        assert!(f(2, &[1, 2, 1, 2, 2]) == true);
        assert!(f(1, &[1, 2, 1, 2, 2]) == false);
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let yes = f(k, &a);
    println!("{}", if yes { "Yes" } else { "No" });
}
