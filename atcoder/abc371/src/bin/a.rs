use proconio::input;

fn main() {
    input! {
        sab: char,
        sac: char,
        sbc: char,
    }
    let result = if sab == '>' {
        if sac == '>' {
            if sbc == '>' {
                'B'
            } else {
                'C'
            }
        } else {
            'A'
        }
    } else {
        if sac == '>' {
            'A'
        } else {
            if sbc == '>' {
                'C'
            } else {
                'B'
            }
        }
    };
    println!("{result}");
}
