pub fn comb_sort<T: Ord>(arr: &mut [T]) {
    let mut gap = arr.len();
    let shrink = 1.247;
    let mut sorted = false;
    while !sorted {
        gap = (gap as f32 / shrink).floor() as usize;
        if gap <= 1 {
            gap = 1;
            sorted = true;
        }
        for i in 0..arr.len()-gap {
            let j = i + gap;
            if arr[i] > arr[j] {
                arr.swap(i, j);
                sorted = false;
            }
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descending() {
        let mut v = vec![6, 4, 5, 1, 2, 3];
        comb_sort(&mut v);
        for i in 0..v.len()-1 {
            assert!(v[i] <= v[i+1]);
        }
    }

    #[test]
    fn ascending() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        comb_sort(&mut v);
        for i in 0..v.len()-1 {
            assert!(v[i] <= v[i+1]);
        }
    }
}