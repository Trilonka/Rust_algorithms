pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let mut sorted = false;
    let mut n = arr.len();
    while !sorted {
        sorted = true;
        for i in 0..n-1 {
            if arr[i] > arr[i+1] {
                arr.swap(i, i+1);
                sorted = false;
            }
        }
        n -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn descending() {
        let mut v = vec![6, 5, 4, 3, 2, 1];
        bubble_sort(&mut v);
        for i in 0..v.len()-1 {
            assert!(v[i] <= v[i+1]);
        }
    }

    #[test]
    fn ascending() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        bubble_sort(&mut v);
        for i in 0..v.len()-1 {
            assert!(v[i] <= v[i+1]);
        }
    }
}