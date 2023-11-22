use proconio::input;

fn insertion_sort(a: &mut Vec<i32>) {
    let n = a.len();

    for i in 1..n {
        let v = a[i];
        let mut j = i;

        while j > 0 {
            if a[j - 1] > v {
                a[j] = a[j - 1];
            } else {
                break;
            }

            j -= 1;
        }
        a[j] = v;
    }
}

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }

    insertion_sort(&mut a);
}
