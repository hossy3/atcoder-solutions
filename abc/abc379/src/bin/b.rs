use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        k: usize,
        mut s: Chars,
    }

    let mut result = 0usize;
    loop {
        if let Some(i) = (0..=(s.len() - k)).find(|&i| s[i..(i + k)].iter().all(|&x| x == 'O')) {
            for i in i..(i + k) {
                s[i] = 'X';
            }
            result += 1;
        } else {
            break;
        }
    }
    println!("{result}");
}
