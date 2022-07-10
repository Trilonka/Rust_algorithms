pub fn radix_sort(arr: &mut [u64]) {
    if arr.len() < 2 {
        return;
    }
    let max: usize = match arr.iter().max() {
        Some(&x) => x as usize,
        None => return,
    };
    let radix = arr.len().next_power_of_two();
    let mut place = 1;
    while place <= max {
        let digit_of = |x| x as usize / place % radix;
        let mut counter = vec![0; radix];
        for &x in arr.iter() {
            counter[digit_of(x)] += 1;
        }
        for i in 1..radix {
            counter[i] += counter[i-1];
        }
        for &x in arr.to_owned().iter().rev() {
            counter[digit_of(x)] -= 1;
            arr[counter[digit_of(x)]] = x;
        }
        place *= radix;
    }
}

mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut arr = vec![329, 457, 657, 839, 436, 720, 355];
        radix_sort(&mut arr);
        assert_eq!(arr, vec![329, 355, 436, 457, 657, 720, 839]);
    }

    #[test]
    fn empty() {
        let mut arr = vec![];
        radix_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn one_element() {
        let mut arr = vec![1];
        radix_sort(&mut arr);
        assert_eq!(arr, vec![1]);
    }

    #[test]
    fn pre_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        radix_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }
}