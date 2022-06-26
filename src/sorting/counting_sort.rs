pub fn counting_sort(arr: &mut [u32], maxval: usize) {
    let mut occurences: Vec<u32> = vec![0; maxval+1];
    for &data in arr.iter() {
        occurences[data as usize] += 1;
    }
    let mut i = 0;
    for (data, &number) in occurences.iter().enumerate() {
        for _ in 0..number {
            arr[i] = data as u32;
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descending() {
        let mut v = vec![6, 4, 5, 1, 2, 3];
        counting_sort(&mut v, 6);
        for i in 0..v.len()-1 {
            assert!(v[i] <= v[i+1]);
        }
    }

    #[test]
    fn ascending() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        counting_sort(&mut v, 6);
        for i in 0..v.len()-1 {
            assert!(v[i] <= v[i+1]);
        }
    }
}