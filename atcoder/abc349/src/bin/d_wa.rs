use proconio::input;

trait BitTest {
    fn bit_test(&self, i: u32) -> bool;
}

impl BitTest for usize {
    fn bit_test(&self, i: u32) -> bool {
        self & (1 << i) != 0
    }
}

fn f(l: usize, r: usize) -> Vec<usize> {
    let mut v = vec![l];
    let k = r.ilog2();

    let mut x = l;
    for i in 0..k {
        if x.bit_test(i) {
            x += 1 << i;
            v.push(x);
        }
    }

    let m = 1usize << r.ilog2();
    if *v.last().unwrap() != m {
        v.push(m);
        x = m;
    }

    for i in (0..k).rev() {
        if r.bit_test(i) {
            x += 1 << i;
            v.push(x);
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
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
    }
}
