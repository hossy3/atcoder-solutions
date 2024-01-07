use proconio::input;

fn f(l: usize, k: usize, v: &[usize]) -> bool {
    let mut num = 0;
    let mut cur = 0;
    for &x in v {
        cur += x;
        if cur >= l {
            num += 1;
            cur = 0;
        }
    }
    num > k
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_func() {
        assert_eq!(f(12, 2, &[8, 5, 13, 8]), true);
        assert_eq!(f(13, 2, &[8, 5, 13, 8]), true);
        assert_eq!(f(14, 2, &[8, 5, 13, 8]), false);
    }
}

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        a: [usize; n],
    }

    let mut v = Vec::with_capacity(n + 1);
    v.push(a[0]);
    for x in a.windows(2) {
        v.push(x[1] - x[0]);
    }
    v.push(l - a[n - 1]);

    let v0: Vec<usize> = (1..=l).collect();
    let result = v0.partition_point(|&i| f(i, k, &v));
    println!("{result}");
}
