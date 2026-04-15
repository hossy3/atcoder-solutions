use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [u8; n],
    }

    let mut result = 0;
    for i in 0..n {
        if s[i] == 0
            && !((i > 1 && s[i - 2] == 1) && (i > 0 && s[i - 1] == 1))
            && !((i > 0 && s[i - 1] == 1) && (i < n - 1 && s[i + 1] == 1))
            && !((i < n - 1 && s[i + 1] == 1) && (i < n - 2 && s[i + 2] == 1))
        {
            result += 1;
            s[i] = 1;
        }
    }
    println!("{result}");
}
