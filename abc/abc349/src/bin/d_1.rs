use proconio::input;

trait BitTest {
    fn bit_test(&self, i: u32) -> bool;
}

impl BitTest for usize {
    fn bit_test(&self, i: u32) -> bool {
        self & (1 << i) != 0
    }
}

fn f(mut l: usize, mut r: usize) -> Vec<usize> {
    let mut v = vec![l, r];
    let k = r.ilog2();

    for i in 0..k {
        if l.bit_test(i) {
            l += 1 << i;
            v.push(l);
        }
        if r.bit_test(i) {
            r -= 1 << i;
            v.push(r);
        }
        if l >= r {
            break;
        }
    }
    v.sort();
    v.dedup();

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(f(0, 1), vec![0, 1]);
        assert_eq!(f(0, 2), vec![0, 2]);
        assert_eq!(f(0, 3), vec![0, 2, 3]);
        assert_eq!(f(0, 4), vec![0, 4]);
        assert_eq!(f(0, 5), vec![0, 4, 5]);

        assert_eq!(f(1, 2), vec![1, 2]);
        assert_eq!(f(1, 3), vec![1, 2, 3]);
        assert_eq!(f(1, 4), vec![1, 2, 4]);
        assert_eq!(f(1, 5), vec![1, 2, 4, 5]);
        assert_eq!(f(1, 6), vec![1, 2, 4, 6]);

        assert_eq!(f(2, 3), vec![2, 3]);
        assert_eq!(f(2, 4), vec![2, 4]);
        assert_eq!(f(2, 5), vec![2, 4, 5]);
        assert_eq!(f(2, 6), vec![2, 4, 6]);
        assert_eq!(f(2, 7), vec![2, 4, 6, 7]);

        assert_eq!(f(3, 4), vec![3, 4]);
        assert_eq!(f(3, 5), vec![3, 4, 5]);
        assert_eq!(f(3, 6), vec![3, 4, 6]);
        assert_eq!(f(3, 7), vec![3, 4, 6, 7]);
        assert_eq!(f(3, 8), vec![3, 4, 8]);

        assert_eq!(f(4, 5), vec![4, 5]);
        assert_eq!(f(4, 6), vec![4, 6]);
        assert_eq!(f(4, 7), vec![4, 6, 7]);
        assert_eq!(f(4, 8), vec![4, 8]);
        assert_eq!(f(4, 9), vec![4, 8, 9]);

        assert_eq!(f(0, 1024), vec![0, 1024]);
        assert_eq!(f(0, 1027), vec![0, 1024, 1026, 1027]);
        assert_eq!(f(254, 1027), vec![254, 256, 512, 1024, 1026, 1027]);
        assert_eq!(f(192, 1604), vec![192, 256, 512, 1024, 1536, 1600, 1604]);
        assert_eq!(f(1024, 2048), vec![1024, 2048]);
        assert_eq!(f(0, 1 << 60), vec![0, 1 << 60]);
        assert_eq!(f(1, 8), vec![1, 2, 4, 8]);
        assert_eq!(f(1, 15), vec![1, 2, 4, 8, 12, 14, 15]);
        assert_eq!(f(1022, 1024), vec![1022, 1024]);
        assert_eq!(f(1023, 1024), vec![1023, 1024]);
        assert_eq!(f(1024, 1025), vec![1024, 1025]);
        assert_eq!(f(1024, 1026), vec![1024, 1026]);
        assert_eq!(f(1022, 1025), vec![1022, 1024, 1025]);
    }
}

fn main() {
    input! {
        l: usize,
        r: usize,
    }
    let v = f(l, r);
    println!("{}", v.len() - 1);
    for v in v.windows(2) {
        println!("{} {}", v[0], v[1]);
        eprintln!("{:x} {:x}", v[0], v[1]);
    }
}
