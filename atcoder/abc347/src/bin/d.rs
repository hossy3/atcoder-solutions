use proconio::input;

fn f(a: i64, b: i64, c: i64) -> Option<(i64, i64)> {
    let pc = c.count_ones() as i64;
    if (a + b - pc) % 2 != 0 {
        return None;
    }
    if a + b < pc {
        return None;
    }

    let mut x = c;
    let mut y = 0i64;
    for i in 0..60 {
        let k = 1i64 << i;
        if x & k != 0 && x.count_ones() as i64 - a != y.count_ones() as i64 - b {
            x -= k;
            y += k;
        }
    }
    if x.count_ones() as i64 == a && y.count_ones() as i64 == b {
        return Some((x, y));
    }

    for i in 0..60 {
        let k = 1i64 << i;
        if c & k == 0 {
            x += k;
            y += k;
            if x.count_ones() as i64 == a && y.count_ones() as i64 == b {
                return Some((x, y));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(f(0, 0, 0), Some((0, 0)));
        assert_eq!(f(0, 0, 2), None);
        assert_eq!(f(1, 1, 0), Some((1, 1)));
        assert_eq!(f(2, 2, 0), Some((3, 3)));
    }
}

fn main() {
    input! {
        (a, b, c): (i64, i64, i64),
    }
    if let Some((x, y)) = f(a, b, c) {
        println!("{x} {y}");
    } else {
        println!("-1");
    }
}
