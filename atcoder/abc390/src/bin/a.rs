use proconio::input;

fn f(a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> bool {
    a1 == 1 && a2 == 2 && a3 == 3 && a4 == 4 && a5 == 5
}

fn main() {
    input! {
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
    }
    let yes = f(a2, a1, a3, a4, a5)
        || f(a1, a3, a2, a4, a5)
        || f(a1, a2, a4, a3, a5)
        || f(a1, a2, a3, a5, a4);
    println!("{}", if yes { "Yes" } else { "No" });
}
