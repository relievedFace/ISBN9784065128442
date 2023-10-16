use proconio::input;

fn main() {
    input! {
        n: usize,
        v: i32,
        a: [i32; n],
    }

    let mut found_id = None;

    for i in 0..n {
        if a[i] == v {
            found_id = Some(i);
            break;
        }
    }

    if let Some(found_id) = found_id {
        println!("{}", found_id);
    } else {
        println!("-1");
    }
}
