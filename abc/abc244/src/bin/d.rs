use proconio::input;

fn main() {
    input! {
        s1: char,
        s2: char,
        s3: char,
        t1: char,
        t2: char,
        t3: char,
    }
    let yes = ((s1 == t1) && (s2 == t2) && (s3 == t3)) || ((s1 != t1) && (s2 != t2) && (s3 != t3));
    println!("{}", if yes { "Yes" } else { "No" });
}
