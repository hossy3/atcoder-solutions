use proconio::input;

// i の範囲を返す (斜めにたどる)
fn f(h: usize, w: usize, i: usize) -> (usize, usize) {
    let h = h as i64;
    let w = w as i64;
    let i = i as i64;
    let mut i0 = 0;
    let mut i1 = h;

    let j = i - i0;
    if j < 0 {
        i0 -= j;
    } else if j >= w {
        i0 += j - w + 1;
    }
    let j = i + 1 - i1;
    if j < 0 {
        i1 += j;
    } else if j >= w {
        i1 += j - w + 1;
    }
    (i0 as usize, i1 as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(f(2, 2, 2), (1, 2));
        assert_eq!(f(2, 2, 1), (0, 2));
        assert_eq!(f(2, 2, 0), (0, 1));

        assert_eq!(f(4, 7, 0), (0, 1));
        assert_eq!(f(4, 7, 1), (0, 2));
        assert_eq!(f(4, 7, 2), (0, 3));
        assert_eq!(f(4, 7, 3), (0, 4));
        assert_eq!(f(4, 7, 4), (0, 4));
        assert_eq!(f(4, 7, 5), (0, 4));
        assert_eq!(f(4, 7, 6), (0, 4));
        assert_eq!(f(4, 7, 7), (1, 4));
        assert_eq!(f(4, 7, 8), (2, 4));
        assert_eq!(f(4, 7, 9), (3, 4));

        assert_eq!(f(7, 4, 0), (0, 1));
        assert_eq!(f(7, 4, 1), (0, 2));
        assert_eq!(f(7, 4, 2), (0, 3));
        assert_eq!(f(7, 4, 3), (0, 4));
        assert_eq!(f(7, 4, 4), (1, 5));
        assert_eq!(f(7, 4, 5), (2, 6));
        assert_eq!(f(7, 4, 6), (3, 7));
        assert_eq!(f(7, 4, 7), (4, 7));
        assert_eq!(f(7, 4, 8), (5, 7));
        assert_eq!(f(7, 4, 9), (6, 7));
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i64; w]; h],
        p: [i64; h + w - 1],
    }

    let mut v = vec![vec![i64::MAX; w]; h]; // コイン現在必要
    let x = (p[h + w - 2] - a[h - 1][w - 1]).max(0);
    v[h - 1][w - 1] = x;

    for i in (0..(h + w - 2)).rev() {
        let (i0, i1) = f(h, w, i);
        for i0 in i0..i1 {
            let j0 = i - i0;
            if i0 < h - 1 {
                let x0 = (p[i] + v[i0 + 1][j0] - a[i0][j0]).max(0);
                v[i0][j0] = x0.min(v[i0][j0]);
            }
            if j0 < w - 1 {
                let x0 = (p[i] + v[i0][j0 + 1] - a[i0][j0]).max(0);
                v[i0][j0] = x0.min(v[i0][j0]);
            }
        }
    }

    let result = v[0][0];
    println!("{result}");
}
