use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut t = None;
    for c in s {
        if let Some((c0, count)) = t {
            if c == c0 {
                t = Some((c, count + 1));
            } else {
                print!("{c0}");
                if count > 1 {
                    print!("{count}");
                }
                t = Some((c, 1));
            }
        } else {
            t = Some((c, 1usize));
        }
    }

    if let Some((c0, count)) = t {
        print!("{c0}");
        if count > 1 {
            print!("{count}");
        }
    }
}
