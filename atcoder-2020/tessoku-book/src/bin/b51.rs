use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut stack = Vec::new();
    for (i, c) in s.iter().enumerate() {
        let i = i + 1;
        if *c == '(' {
            stack.push(i);
        } else {
            if let Some(j) = stack.pop() {
                println!("{} {}", j, i);
            }
        }
    }
}
