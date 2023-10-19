use proconio::input;

fn main() {
    println!("Start Game!");

    let mut left = 20;
    let mut right = 36;

    while right - left > 1 {
        let mid = left + (right - left) / 2;

        println!("Is the age less than {}?(yes/no)", mid);

        input! {
            ans: String,
        }

        if ans == "yes" {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("The age is {}!", left);
}
