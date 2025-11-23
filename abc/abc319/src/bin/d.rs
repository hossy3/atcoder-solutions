use proconio::input;

// bin-search

fn f(w: usize, m: usize, l: &[usize]) -> bool {
    let mut lines = 1usize;
    let mut c = 0usize;
    for &x in l {
        if x > w {
            return false;
        }
        if c == 0 {
            c = x;
        } else if c + 1 + x > w {
            c = x;
            lines += 1;
            if lines > m {
                return false;
            }
        } else {
            c += 1 + x;
        }
    }
    true
}

#[test]
fn test_func() {
    let l = vec![9, 5, 2, 7, 1, 8, 8, 2, 1, 5, 2, 3, 6];
    assert_eq!(f(25, 3, &l), false);
    assert_eq!(f(26, 3, &l), true);
    assert_eq!(f(27, 3, &l), true);
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ls: [usize; n],
    }

    let mut l = 1;
    let mut r = 200_001 * 1_000_000_000;
    while l != r {
        let w = (l + r) / 2;
        if f(w, m, &ls) {
            r = w;
        } else {
            l = w + 1;
        }
    }
    println!("{l}");
}
