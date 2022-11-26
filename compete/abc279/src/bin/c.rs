use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
        t: [Chars; h],
    }

    let mut a0 = vec![String::new(); w];
    for line in &s {
        for i in 0..line.len() {
            a0[i].push(line[i]);
        }
    }
    a0.sort();

    let mut a1 = vec![String::new(); w];
    for line in &t {
        for i in 0..line.len() {
            a1[i].push(line[i]);
        }
    }
    a1.sort();

    let yes = a0.eq(&a1);
    println!("{}", if yes { "Yes" } else { "No" });
}
