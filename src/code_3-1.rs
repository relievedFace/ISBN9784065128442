use proconio::input;

fn main() {
    input! {
        n: usize,
        v: i32,
        a: [i32; n],
    }

    let mut exist = false;

    for i in 0..n {
        if a[i] == v {
            exist = true;
            break;
        }
    }

    if exist {
        println!("Yes");
    } else {
        println!("No");
    }
}
