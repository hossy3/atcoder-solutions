use proconio::input;

const N: usize = 32;

fn build_squares(h: usize, w: usize) -> [usize; N] {
    let mut a = [0usize; N];
    let mut h0 = h;
    let mut w0 = w;
    for i in (0..N).rev() {
        let k = 1usize << i;
        if h0 / k > 0 {
            a[i] += (h - h0) / k;
        }
        if w0 / k > 0 {
            a[i] += (w - w0) / k;
        }
        if h0 / k > 0 && w0 / k > 0 {
            a[i] += 1;
        }
        h0 = h0 % k;
        w0 = w0 % k;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3_7() {
        assert_eq!(
            build_squares(3, 7),
            [
                9, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn test_4_4() {
        assert_eq!(
            build_squares(4, 4),
            [
                0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn test_5_7() {
        assert_eq!(
            build_squares(5, 7),
            [
                11, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0
            ]
        );
    }
}

fn f(h: usize, w: usize, mut a: Vec<usize>) -> bool {
    a.sort();
    a.reverse();

    let mut squares = build_squares(h, w);
    for i in a {
        if squares[i] == 0 {
            for j in (i..(N - 1)).rev() {
                squares[j] += squares[j + 1] * 4;
                squares[j + 1] = 0;
            }
        }
        if squares[i] == 0 {
            return false;
        }
        squares[i] -= 1;
    }
    true
}

fn main() {
    input! {
        (h, w, n): (usize, usize, usize),
        a: [usize; n],
    }
    let yes = f(h, w, a);
    println!("{}", if yes { "Yes" } else { "No" });
}
