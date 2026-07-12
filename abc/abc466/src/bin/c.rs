use proconio::input_interactive;

fn main() {
    input_interactive! {
        n: usize,
    }

    let mut l = 1;
    let mut r = 2;
    let mut result = 0;

    while l < n {
        println!("? {l} {r}");
        input_interactive! {
            s: String,
        }

        if s.as_str() == "Yes" {
            if r == n {
                result += r - l;
                l += 1;
            } else {
                r += 1;
            }
        } else {
            result += r - l - 1;
            if r == l + 1 {
                r += 1;
                l += 1;
            } else {
                l += 1;
            }
        }
    }

    println!("! {result}");
}
