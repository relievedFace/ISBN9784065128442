use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    for i in (2..=n).step_by(2) {
        println!("{}", i)
    }
}
