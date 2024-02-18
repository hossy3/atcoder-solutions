use proconio::input;

const N: usize = 26;

fn build_squares(h: usize, w: usize) -> [usize; N] {
    let mut a = [0usize; N];
    for i in 0..N {
        let k = 1usize << i;
        a[i] = (h / k) * (w / k);
    }
    for i in 0..(N - 1) {
        a[i] -= a[i + 1] * 4;
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
            [9, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );
    }

    #[test]
    fn test_4_4() {
        assert_eq!(
            build_squares(4, 4),
            [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );
    }

    #[test]
    fn test_5_7() {
        assert_eq!(
            build_squares(5, 7),
            [11, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,]
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
