use proconio::input;

fn merge_sort(a: &mut Vec<i32>, left: usize, right: usize) {
    if right - left == 1 {
        return;
    }

    let mid = left + (right - left) / 2;

    merge_sort(a, left, mid);
    merge_sort(a, mid, right);

    let mut buf = vec![];

    for ai in a[left..mid].iter() {
        buf.push(*ai)
    }
    for ai in a[mid..right].iter().rev() {
        buf.push(*ai);
    }

    let mut index_left = 0;
    let mut index_right = buf.len() - 1;

    for ai in a[left..right].iter_mut() {
        if buf[index_left] <= buf[index_right] {
            *ai = buf[index_left];
            index_left += 1;
        } else {
            *ai = buf[index_right];
            index_right -= 1;
        }
    }
}

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }

    merge_sort(&mut a, 0, n);
}
