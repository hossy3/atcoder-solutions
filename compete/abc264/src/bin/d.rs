use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }
    let mut count = 0;
    let x = s.iter().position(|&c| c == 'a').unwrap();
    if x > 0 {
        for i in (0..x).rev() {
            s.swap(i, i + 1);
            count += 1;
        }
    }
    let x = s.iter().position(|&c| c == 't').unwrap();
    if x > 1 {
        for i in (1..x).rev() {
            s.swap(i, i + 1);
            count += 1;
        }
    }
    let x = s.iter().position(|&c| c == 'c').unwrap();
    if x > 2 {
        for i in (2..x).rev() {
            s.swap(i, i + 1);
            count += 1;
        }
    }
    let x = s.iter().position(|&c| c == 'o').unwrap();
    if x > 3 {
        for i in (3..x).rev() {
            s.swap(i, i + 1);
            count += 1;
        }
    }
    let x = s.iter().position(|&c| c == 'd').unwrap();
    if x > 4 {
        for i in (4..x).rev() {
            s.swap(i, i + 1);
            count += 1;
        }
    }
    let x = s.iter().position(|&c| c == 'e').unwrap();
    if x > 5 {
        for i in (5..x).rev() {
            s.swap(i, i + 1);
            count += 1;
        }
    }
    println!("{}", count);
}
