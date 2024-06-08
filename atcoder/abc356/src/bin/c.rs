use proconio::{input, marker::Usize1};

fn f(i: usize, k: usize, v: &[(usize, bool)]) -> bool {
    for &(j, b) in v {
        let k0 = (i & j).count_ones() as usize;
        if (k0 < k && b) || (k0 >= k && !b) {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let mut v = vec![];
    for _ in 0..m {
        input! {
            c: usize,
            a: [Usize1; c],
            r: char,
        }
        let mut i = 0usize;
        for x in a {
            i += 1 << x;
        }
        let b = r == 'o';
        v.push((i, b));
    }

    let result = (0..(1 << n)).filter(|&i| f(i, k, &v)).count();
    println!("{result}");
}
