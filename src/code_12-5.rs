use proconio::input;

const MAX: usize = 100000;

fn bucket_sort(a: &mut Vec<i32>) {
    let n = a.len();
    let mut num = vec![0; MAX];

    for ai in a.iter() {
        num[*ai as usize] += 1;
    }

    let mut sum = vec![0; MAX];

    for v in 1..MAX {
        sum[v] = sum[v - 1] + num[v];
    }

    let mut a2 = vec![0; n];

    for ai in a.iter().rev() {
        sum[*ai as usize] -= 1;
        a2[sum[*ai as usize]] = *ai;
    }

    *a = a2;
}

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }

    bucket_sort(&mut a);
}
