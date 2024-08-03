use proconio::input;

fn main() {
    input! {
        y: usize,
    }
    let yes = (y % 4 == 0) && ((y % 100 != 0) || (y % 400 == 0));  
    println!("{}", if yes { 366 } else { 365 });
}
