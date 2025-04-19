use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut error_count = 0usize;
    let mut login = false;
    for s in s {
        match s.as_str() {
            "login" => {
                login = true;
            }
            "logout" => {
                login = false;
            }
            "public" => {}
            "private" => {
                if !login {
                    error_count += 1;
                }
            }
            _ => {
                unreachable!()
            }
        }
    }

    println!("{error_count}");
}
