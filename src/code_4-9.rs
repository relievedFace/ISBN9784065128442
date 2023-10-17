use proconio::input;

fn func(i: usize, w: i32, a: &Vec<i32>) -> bool {
    if i == 0 {
        if w == 0 {
            true
        } else {
            false
        }
    } else if func(i - 1, w, a) {
        true
    } else if func(i - 1, w - a[i - 1], a) {
        true
    } else {
        false
    }
}

fn main() {
    input! {
        n: usize,
        w: i32,
        a: [i32; n],
    }

    if func(n, w, &a) {
        println!("Yes");
    } else {
        println!("No");
    }
}
