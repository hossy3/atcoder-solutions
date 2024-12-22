use proconio::input;

fn f(a: &[usize]) -> usize {
    let n = a.len();
    let mut result = 0usize;
    let mut v = [0; 200_001];
    let mut j = 0;
    for i in 0..(n / 2) {
        while j < (n / 2) {
            let b = a[j * 2] == a[j * 2 + 1] && v[a[j * 2]] == 0;
            if !b {
                if i == j {
                    j += 1;
                }
                break;
            }
            v[a[j * 2]] += 1;
            result = result.max(((j + 1) - i) * 2);
            j += 1;
        }
        if a[i * 2] == a[i * 2 + 1] {
            v[a[i * 2]] -= 1;
        }
    }
    result
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let result = f(&a).max(f(&a[1..]));
    println!("{result}");
}
