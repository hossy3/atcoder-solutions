use proconio::input;

fn f(n: usize, s: isize, a: &[isize]) -> bool {
    let loop_sum: isize = a.iter().sum();
    let s = s % loop_sum;
    if s == 0 {
        return true;
    }

    let mut sum = 0isize;
    let mut j = 0;
    for i in 0..n {
        while j <= n * 2 {
            if sum == s {
                return true;
            }
            if sum > s {
                break;
            }
            sum += a[j % n];
            j += 1;
        }
        sum -= a[i];
    }
    false
}

fn main() {
    input! {
        n: usize,
        s: isize,
        a: [isize; n],
    }
    let yes = f(n, s, &a);
    println!("{}", if yes { "Yes" } else { "No" });
}
