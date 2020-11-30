fn qsort<T>(a: &Vec<T>) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    if a.len() == 0 {
        return vec![];
    }
    let mid = a[0];
    let left: Vec<T> = a[1..].iter().filter(|s| **s <= mid).cloned().collect();
    let right: Vec<T> = a[1..].iter().filter(|s| **s > mid).cloned().collect();
    let left = qsort(&left);
    let right = qsort(&right);
    let mut res = vec![];
    res.extend(left);
    res.push(mid);
    res.extend(right);
    res
}

fn quick_sort<T>(a: &mut Vec<T>, left: usize, right: usize)
where
    T: PartialOrd + Copy,
{
    if left >= right {
        return;
    }
    let mut l = left;
    let mut r = right;
    while l < r {
        while l < r && a[r] >= a[left] {
            r -= 1;
        }
        while l < r && a[l] <= a[left] {
            l += 1;
        }
        a.swap(l, r);
    }
    a.swap(left, l);
    if l > 1 {
        quick_sort(a, left, l - 1);
    }
    quick_sort(a, r + 1, right);
}

fn main() {
    let mut a = vec![1, 4, 2, 8, 5, 7];
    let b = vec![1, 2, 4, 5, 7, 8];
    assert_eq!(qsort(&a), b);

    let n = a.len();
    quick_sort(&mut a, 0, n - 1);
    assert_eq!(a, b);
}
