use proconio::input;

fn main() {
    input! {
        q: usize,
        a: [usize; q],
    }
    let mut volume = 0isize;
    let mut playing = false;
    for x in a {
        match x {
            1 => {
                volume += 1;
            }
            2 => {
                volume = (volume - 1).max(0);
            }
            3 => {
                playing = !playing;
            }
            _ => unreachable!(),
        }

        let yes = playing && volume >= 3;
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
