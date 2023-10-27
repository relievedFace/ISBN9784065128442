use proconio::input;

const VALUE: [i32; 6] = [500, 100, 50, 10, 5, 1];

fn main() {
    input! {
        mut x: i32,
        a: [i32; 6],
    }

    let mut result = 0;

    for i in 0..6 {
        let mut add = x / VALUE[i];

        if add > a[i] {
            add = a[i];
        }

        x -= VALUE[i] * add;
        result += add;
    }

    println!("{}", result);
}
