use proconio::input;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(f(0, 0, 0, 0), 0);
        assert_eq!(f(0, 0, 1, 0), 0);
        assert_eq!(f(0, 0, 2, 0), 1);
        assert_eq!(f(0, 0, 3, 0), 1);
        assert_eq!(f(0, 0, 4, 0), 2);
        
        assert_eq!(f(0, 0, 0, 1), 1);
        assert_eq!(f(0, 0, 1, 1), 1);
        assert_eq!(f(0, 0, 2, 1), 1);
        assert_eq!(f(0, 0, 3, 1), 2);
        assert_eq!(f(0, 0, 4, 1), 2);
        
        assert_eq!(f(0, 0, 0, 2), 2);
        assert_eq!(f(0, 0, 1, 2), 2);
        assert_eq!(f(0, 0, 2, 2), 2);
        assert_eq!(f(0, 0, 3, 2), 2);
        assert_eq!(f(0, 0, 4, 2), 3);
    }
}

fn f(mut sx: i64, sy: i64, mut tx: i64, ty: i64) -> u64 {
    // 長方形の右側に寄せる
    if (sx + sy) % 2 == 0 {
        sx += 1;
    }
    if (tx + ty) % 2 == 0 {
        tx += 1;
    }

    let dx = tx.abs_diff(sx);
    let dy = ty.abs_diff(sy);
    if dy >= dx {
        dy
    } else {
        dy + (dx - dy) / 2
    }
}

fn main() {
    input! {
        sx: i64,
        sy: i64,
        tx: i64,
        ty: i64,
    }
    let result = f(sx, sy, tx, ty);
    println!("{result}");
}
