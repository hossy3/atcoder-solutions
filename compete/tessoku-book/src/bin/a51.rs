use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut stack = vec![];

    for _ in 0..q {
        input! {
            k: usize,
        }

        match k {
            1 => {
                input! {
                    name: String,
                }
                stack.push(name);
            }
            2 => {
                if let Some(name) = stack.last() {
                    println!("{}", name);
                }
            }
            _ => {
                stack.pop();
            }
        }
    }
}
