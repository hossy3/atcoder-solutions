use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut r = 0;
    let mut count = 0;
    let mut count_o = 0;
    let mut count_x = 0;
    for l in 0..n {
        while r < n && (count_o == 0 || count_x == 0) {
            if s[r] == 'o' {
                count_o += 1;
            } else {
                count_x += 1;
            }
            r += 1;
        }
        if count_o == 0 || count_x == 0 {
            break;
        }
        count += n - r + 1;
        if s[l] == 'o' {
            count_o -= 1;
        } else {
            count_x -= 1;
        }
    }
    println!("{}", count);
}
