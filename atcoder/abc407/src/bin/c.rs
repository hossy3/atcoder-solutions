use proconio::{input, marker::Chars};

// a を b にするためにボタンを押す回数
fn f(a: char, b: char) -> usize {
    if a > b {
        10 + b as usize - a as usize
    } else {
        b as usize - a as usize
    }
}

fn main() {
    input! {
        s: Chars,
    }

    // 最後の1文字以外を進める
    let mut count = 0usize;
    for v in s.windows(2) {
        count += 1; // ボタン A を押す
        count += f(v[1], v[0]); // ボタン B を押す
    }

    // 最後の1文字
    count += 1; // ボタン A を押す
    count += f('0', s[s.len() - 1]); // ボタン B を押す

    println!("{count}");
}
