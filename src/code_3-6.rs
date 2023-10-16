use proconio::input;

fn main() {
    input! {
        n: usize,
        w: i32,
        a: [i32; n],
    }

    let mut exist = false;

    for bit in 0..1 << n {
        let mut sum = 0;
        for i in 0..n {
            if (bit & 1 << i) > 0 {
                sum += a[i];
            }
        }

        if sum == w {
            exist = true;
            // 本に記載されているコードはbreakしていない
            break;
        }
    }

    if exist {
        println!("Yes");
    } else {
        println!("No");
    }
}
