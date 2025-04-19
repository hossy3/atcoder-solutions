use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut stack = vec![0; 100];
    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {
                    x: usize,
                }
                stack.push(x);
            }
            2 => {
                let Some(x) = stack.pop() else { unreachable!() };
                println!("{x}");
            }
            _ => unreachable!(),
        }
    }
}
