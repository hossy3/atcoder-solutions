use proconio::{input, marker::Chars};

fn f(s: &[char]) -> usize {
    5 - s.iter().filter(|&&c| c == 'o').count()
}

fn main() {
    input! {
        s: [Chars; 5],
    }

    let mut result = 5;
    for i in 0..5 {
        result = result.min(f(&s[i]));
    }
    for i in 0..5 {
        result = result.min(f(&[s[0][i], s[1][i], s[2][i], s[3][i], s[4][i]]));
    }
    result = result.min(f(&[s[0][0], s[1][1], s[2][2], s[3][3], s[4][4]]));
    result = result.min(f(&[s[0][4], s[1][3], s[2][2], s[3][1], s[4][0]]));
    println!("{result}");
}
