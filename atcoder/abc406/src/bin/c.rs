use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut result = 0usize;
    let mut i = 0;
    while i < n {
        let mut j = i;
        while j + 1 < n && p[j] < p[j + 1] {
            j += 1;
        }
        if j == n - 1 {
            break;
        }
        if i == j {
            i += 1;
            continue;
        }

        let mut k = j;
        while k + 1 < n && p[k] > p[k + 1] {
            k += 1;
        }
        if k == n - 1 {
            break;
        }
        if j == k {
            i = k;
            continue;
        }

        let mut l = k;
        while l + 1 < n && p[l] < p[l + 1] {
            l += 1;
        }
        result += (j - i) * (l - k);
        i = k;
    }

    println!("{result}");
}
