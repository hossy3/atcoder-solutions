use proconio::input;

fn f(
    px: isize,
    py: isize,
    qx: isize,
    qy: isize,
    rx: isize,
    ry: isize,
    sx: isize,
    sy: isize,
) -> bool {
    // 傾きが違えばどこかに交点がある
    if (qy - py) * (sx - rx) != (qx - px) * (sy - ry) {
        return true;
    }

    // 中点が同一直線上でも良い
    let x0 = px + qx;
    let y0 = py + qy;
    let x1 = rx + sx;
    let y1 = ry + sy;
    if (y1 - y0) * (qy - py) == -(x1 - x0) * (qx - px) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(f(0, 0, 1, 2, 0, 0, -1, -2), false);
        assert_eq!(f(0, 0, 1, 2, 0, 0, 2, 4), true);
    }
}

fn main() {
    input! {
        t: usize,
        pqrs: [(isize, isize, isize, isize,isize, isize, isize, isize); t],
    }
    for (px, py, qx, qy, rx, ry, sx, sy) in pqrs {
        let yes = f(px, py, qx, qy, rx, ry, sx, sy);
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
