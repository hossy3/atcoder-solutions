use proconio::input;

fn f(txt: &str, level: usize, rest: usize) {
    if rest == 0 {
        println!("{txt}");
        return;
    }

    if level < rest {
        f(&(txt.to_string() + "("), level + 1, rest - 1);
    }
    if level > 0 {
        f(&(txt.to_string() + ")"), level - 1, rest - 1);
    }
}

fn main() {
    input! {
        n: usize,
    }
    if n > 0 && n % 2 == 0 {
        f("", 0, n);
    }
}
