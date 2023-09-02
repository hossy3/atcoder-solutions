use proconio::input;

fn f(used: &mut [bool], m: &[Vec<usize>]) -> usize {
    if used.iter().all(|&b| b) {
        return 0;
    }

    let mut result = 0;
    let n = used.len();
    for i in 0..(n - 1) {
        if used[i] {
            continue;
        }
        used[i] = true;
        for j in (i + 1)..n {
            if used[j] {
                continue;
            }
            used[j] = true;
            result = result.max(m[i][j - i - 1] + f(used, m));
            used[j] = false;
        }
        used[i] = false;
        break;
    }

    result
}

fn g(used: &mut [bool], m: &[Vec<usize>]) -> usize {
    let n = used.len();
    if n % 2 == 1 {
        let mut result = 0;
        for i in 0..n {
            used[i] = true;
            result = result.max(f(used, m));
            used[i] = false;
        }
        result
    } else {
        f(used, m)
    }
}

fn main() {
    input! {
        n: usize,
    }

    let mut m = vec![vec![]; n - 1];
    for i in 0..(n - 1) {
        input! {
            d: [usize; n - i - 1],
        }
        m[i] = d;
    }

    let mut used = vec![false; n];
    let result = g(&mut used, &m);
    println!("{result}");
}
