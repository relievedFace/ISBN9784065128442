use proconio::input;

fn main() {
    input! {
        n: usize,
        mut inter: [(i32, i32); n],
    }

    inter.sort_by_key(|k| k.1);

    let mut res = 0;
    let mut current_end_time = 0;

    for &inter_i in &inter {
        if inter_i.0 < current_end_time {
            continue;
        }

        res += 1;
        current_end_time = inter_i.1
    }

    println!("{}", res);
}
