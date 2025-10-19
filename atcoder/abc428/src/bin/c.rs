use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    let mut vec = vec![(0i32, true)]; // (今の値, 有効かどうか)
    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {
                    c: char,
                }
                let Some(&(value, is_valid)) = vec.last() else {
                    unreachable!()
                };
                match c {
                    '(' => {
                        let value = value + 1;
                        vec.push((value, is_valid));
                    }
                    ')' => {
                        let value = value - 1;
                        let is_valid = is_valid && value >= 0;
                        vec.push((value, is_valid));
                    }
                    _ => unreachable!(),
                }
            }
            2 => {
                vec.pop();
            }
            _ => unreachable!(),
        }

        let Some(&(value, is_valid)) = vec.last() else {
            unreachable!()
        };
        let yes = value == 0 && is_valid;
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
