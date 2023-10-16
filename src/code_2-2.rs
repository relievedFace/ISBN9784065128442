use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut count = 0;

    for i in 0..n {
        for j in 0..n {
            count += 1;
        }
    }
}
