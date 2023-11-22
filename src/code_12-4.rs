use proconio::input;

fn heapify(a: &mut Vec<i32>, i: usize, n: usize) {
    let mut child1 = i * 2 + 1;

    if child1 >= n {
        return;
    }

    if child1 + 1 < n && a[child1 + 1] > a[child1] {
        child1 += 1;
    }

    a.swap(i, child1);

    heapify(a, child1, n);
}

fn heap_sort(a: &mut Vec<i32>) {
    let n = a.len();

    for i in (0..n / 2).rev() {
        heapify(a, i, n);
    }

    for i in (1..n).rev() {
        a.swap(0, i);
        heapify(a, 0, i);
    }
}

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }

    heap_sort(&mut a);
}
