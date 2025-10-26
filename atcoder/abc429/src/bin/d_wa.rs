use proconio::input;

fn f(m: usize, c: usize, a: &[usize]) -> usize {
    let n = a.len();

    // それぞれの位置に何人立っているか
    let mut v0 = vec![0usize; n];
    for &x in a {
        v0[x % n] += 1;
    }

    // しゃくとり法 (two-pointer) で余りを処理する
    let mut v1 = vec![0usize; n];

    let mut r = 0usize;
    let mut count = 0usize;
    for l in 0..n {
        while count < c {
            count += v0[(r + 1) % n];
            r = (r + 1) % n;
        }
        v1[l] = count;
        count -= v0[(l + 1) % n];
    }
    eprintln!("{:?}", &v0);
    eprintln!("{:?}", &v1);

    let result = v1.iter().sum::<usize>() * (m / n) + v1[0..(m % n)].iter().sum::<usize>();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(f(1, 2, &[1, 2, 1, 0, 1]), 3);
        assert_eq!(f(2, 2, &[1, 2, 1, 0, 1]), 5);
        assert_eq!(f(3, 2, &[1, 2, 1, 0, 1]), 9);
        assert_eq!(f(4, 2, &[1, 2, 1, 0, 1]), 13);
        assert_eq!(f(5, 2, &[1, 2, 1, 0, 1]), 17);
        assert_eq!(f(6, 2, &[1, 2, 1, 0, 1]), 20);
        assert_eq!(f(7, 2, &[1, 2, 1, 0, 1]), 22);
        assert_eq!(f(8, 2, &[1, 2, 1, 0, 1]), 26);
        assert_eq!(f(9, 2, &[1, 2, 1, 0, 1]), 30);
        assert_eq!(f(10, 2, &[1, 2, 1, 0, 1]), 34);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        c: usize,
        a: [usize; n],
    }

    let result = f(m, c, &a);
    println!("{result}");
}
