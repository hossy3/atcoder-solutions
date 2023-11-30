use proconio::input;

fn f(xa: i64, ya: i64, xb: i64, yb: i64, xc: i64, yc: i64) -> i64 {
    if xa == xb {
        if xb == xc {
            if (yb - ya).signum() == (yc - yb).signum() {
                (yc - ya).abs() - 1
            } else {
                (yb - ya).abs() + (yc - yb).abs() + 3
            }
        } else if yb == yc {
            (yb - ya).abs() + (xc - xb).abs() + 1
        } else {
            if (yb - ya).signum() == (yc - yb).signum() {
                (yc - ya).abs() + (xc - xb).abs() + 1
            } else {
                (yb - ya).abs() + (yc - yb).abs() + (xc - xb).abs() + 3
            }
        }
    } else if ya == yb {
        if yb == yc {
            if (xb - xa).signum() == (xc - xb).signum() {
                (xc - xa).abs() - 1
            } else {
                (xb - xa).abs() + (xc - xb).abs() + 3
            }
        } else if xb == xc {
            (xb - xa).abs() + (yc - yb).abs() + 1
        } else {
            if (xb - xa).signum() == (xc - xb).signum() {
                (xc - xa).abs() + (yc - yb).abs() + 1
            } else {
                (xb - xa).abs() + (xc - xb).abs() + (yc - yb).abs() + 3
            }
        }
    } else {
        if xb == xc {
            if (yb - ya).signum() == (yc - yb).signum() {
                (xc - xa).abs() + (yc - ya).abs() - 1
            } else {
                (xc - xa).abs() + (yb - ya).abs() + (yc - yb).abs() + 1
            }
        } else if yb == yc {
            if (xb - xa).signum() == (xc - xb).signum() {
                (yc - ya).abs() + (xc - xa).abs() - 1
            } else {
                (yc - ya).abs() + (xb - xa).abs() + (xc - xb).abs() + 1
            }
        } else if (xb - xa).signum() == (xc - xb).signum() {
            if (yb - ya).signum() == (yc - yb).signum() {
                (xc - xa).abs() + (yc - ya).abs() + 1
            } else {
                (xc - xa).abs() + (yb - ya).abs() + (yc - yb).abs() + 1
            }
        } else {
            if (yb - ya).signum() == (yc - yb).signum() {
                (yc - ya).abs() + (xb - xa).abs() + (xc - xb).abs() + 1
            } else {
                (xb - xa).abs() + (xc - xb).abs() + (yb - ya).abs() + (yc - yb).abs() + 3
            }
        }
    }
}

#[test]
fn test_func() {
    assert_eq!(f(0, 0, 0, 1, 0, 2), 1);
    assert_eq!(f(0, 0, 0, 1, 0, 0), 5);
    assert_eq!(f(0, 0, 0, 1, 1, 1), 3);
    assert_eq!(f(0, 0, 0, 1, 1, 2), 4);
    assert_eq!(f(0, 0, 0, 1, 1, 0), 6);

    assert_eq!(f(0, 0, 1, 0, 2, 0), 1);
    assert_eq!(f(0, 0, 1, 0, 0, 0), 5);
    assert_eq!(f(0, 0, 1, 0, 1, 1), 3);
    assert_eq!(f(0, 0, 1, 0, 2, 1), 4);
    assert_eq!(f(0, 0, 1, 0, 0, 1), 6);

    assert_eq!(f(0, 0, 1, 1, 1, 2), 2);
    assert_eq!(f(0, 0, 1, 1, 1, 0), 4);
    assert_eq!(f(0, 0, 1, 1, 2, 1), 2);
    assert_eq!(f(0, 0, 1, 1, 0, 1), 4);

    assert_eq!(f(0, 0, 1, 1, 2, 2), 5);
    assert_eq!(f(0, 0, 1, 1, 2, 0), 5);
    assert_eq!(f(0, 0, 1, 1, 0, 2), 5);
    assert_eq!(f(0, 0, 1, 1, 0, 0), 7);
}

fn main() {
    input! {
        xa: i64,
        ya: i64,
        xb: i64,
        yb: i64,
        xc: i64,
        yc: i64,
    }
    let result = f(xa, ya, xb, yb, xc, yc);
    println!("{result}");
}
