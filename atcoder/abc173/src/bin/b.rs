use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut v = [0; 4];
    for s in s {
        match s.as_str() {
            "AC" => { v[0] += 1; },
            "WA" => { v[1] += 1; },
            "TLE" => { v[2] += 1; },
            "RE" => { v[3] += 1; },
            _ => unreachable!(),
        }
    }
    println!("AC x {}", v[0]);
    println!("WA x {}", v[1]);
    println!("TLE x {}", v[2]);
    println!("RE x {}", v[3]);
}
