use proconio::input;

fn f(a: &[Vec<usize>], v: &Vec<bool>, i: usize, b: usize) -> usize {
    if v.iter().all(|&x| x) {
        b
    } else {
        let i = (i..v.len()).find(|&i| !v[i]).unwrap();
        let result = ((i + 1)..v.len())
            .filter(|&j| !v[j])
            .map(|j| {
                let mut v = v.clone();
                v[i] = true;
                v[j] = true;
                f(&a, &v, i + 1, b ^ a[i][j - i - 1])
            })
            .max()
            .unwrap();
        result
    }
}

#[test]
fn test_func() {
    let mut v = vec![false; 4];
    assert_eq!(
        f(&vec![vec![4, 0, 1], vec![5, 3], vec![2]], &mut v, 0, 0),
        6
    );
}

fn main() {
    input! {
        n: usize,
    }
    let mut a = vec![];
    for i in 1..(2 * n) {
        input! {
            a0: [usize; 2 * n - i],
        }
        a.push(a0);
    }
    let mut v = vec![false; n * 2];
    let result = f(&a, &mut v, 0, 0);
    println!("{}", result);
}
