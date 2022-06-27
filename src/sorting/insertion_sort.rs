pub fn insertion_sort<T>(arr: &mut [T])
where 
    T: Ord + Copy,
{
    for i in 1..arr.len() {
        let cur = arr[i];
        let mut j = i-1;
        while arr[j] > cur {
            arr.swap(j+1, j);
            if j == 0 {
                break;
            }
            j-=1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut arr: [u8; 0] = [];
        insertion_sort(&mut arr);
        for i in 0..arr.len()-1.clamp(0, arr.len()) {
            assert!(arr[i] <= arr[i+1])
        }
    }

    #[test]
    fn one_element() {
        let mut arr: [char; 1] = ['a'];
        insertion_sort(&mut arr);
        for i in 0..arr.len()-1 {
            assert!(arr[i] <= arr[i+1])
        }
    }

    #[test]
    fn already_sorted() {
        let mut arr: [&str; 3] = ["a", "b", "c"];
        insertion_sort(&mut arr);
        for i in 0..arr.len()-1 {
            assert!(arr[i] <= arr[i+1])
        }
    }

    #[test]
    fn basic() {
        let mut arr: [&str; 4] = ["d", "a", "c", "b"];
        insertion_sort(&mut arr);
        for i in 0..arr.len()-1 {
            assert!(arr[i] <= arr[i+1])
        }
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr: Vec<&str> = vec!["d", "a", "c", "e", "b"];
        insertion_sort(&mut arr);
        for i in 0..arr.len()-1 {
            assert!(arr[i] <= arr[i+1])
        }
    }

    #[test]
    fn repeated_elements() {
        let mut arr: Vec<usize> = vec![542, 542, 542, 542];
        insertion_sort(&mut arr);
        for i in 0..arr.len()-1 {
            assert!(arr[i] <= arr[i+1])
        }
    }
}