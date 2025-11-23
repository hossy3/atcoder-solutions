use proconio::input;

fn f(dx: i64, dy: i64) -> i64 {
    if dx < dy {
        f(dy, dx)
    } else if dx == dy {
        match dx % 3 {
            0 => 2 * (dx / 3),
            1 => 2 * (dx / 3) + 2,
            2 => 2 * (dx / 3) + 4,
            _ => unreachable!(),
        }
    } else if dy == 0 {
        match dx % 4 {
            0 => 2 * (dx / 4),
            1 => 2 * (dx / 4) + 3,
            2 => 2 * (dx / 4) + 2,
            3 => 2 * (dx / 4) + 3,
            _ => unreachable!(),
        }
    } else if dy == 1 {
        match dx {
            1 => 2,
            2 => 1,
            3 => 2,
            _ => 1 + f(dx - 2, dy - 1),
        }
    } else if dx == 2 * dy {
        dx / 2
    } else if dx < 2 * dy {
        let d = dx - dy;
        d + f(dx - 2 * d, dy - d)
    } else {
        dx / 2 + f(dx - 2 * dy, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        // assert_eq!(f(0, 0), 0);
        // assert_eq!(f(1, 3), 2);
        assert_eq!(f(0, 3), 3);
        assert_eq!(f(3, 3), 2);
    }
}

fn main() {
    input! {
        n: usize,
        rc: [(i64, i64); n],
    }

    let mut sx = 0;
    let mut sy = 0;
    for &(r, c) in &rc {
        sx += r;
        sy += c;
    }
    sx /= n as i64;
    sy /= n as i64;

    let mut result = i64::MAX;
    for dx in -3..=3 {
        for dy in -3..=3 {
            let sx = sx + dx;
            let sy = sy + dy;
            let mut max = 0;
            for &(r, c) in &rc {
                max = max.max(f((sx - r).abs(), (sy - c).abs()));
            }
            result = result.min(max);
        }
    }
    println!("{result}");
}
