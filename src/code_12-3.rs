use proconio::input;

fn quick_sort(a: &mut Vec<i32>, left: usize, right: usize) {
    if right - left <= 1 {
        return;
    }

    let pivot_index = (left + right) / 2;
    let pivot = a[pivot_index];

    a.swap(pivot_index, right - 1);

    let mut i = left;

    for j in left..right - 1 {
        if a[j] < pivot {
            a.swap(i, j);
            i += 1;
        }
    }

    a.swap(i, right - 1);

    quick_sort(a, left, i);
    quick_sort(a, i + 1, right);
}

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }

    quick_sort(&mut a, 0, n);
}
