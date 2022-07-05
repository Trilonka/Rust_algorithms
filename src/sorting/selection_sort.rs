pub fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for left in 0..len {
        let mut smallest = left;
        for right in (left+1)..len {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }
        arr.swap(left, smallest);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        selection_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        selection_sort(&mut res);
        assert_eq!(res, vec!["a", "bb", "cc", "d"]);
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        selection_sort(&mut res);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        selection_sort(&mut res);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        selection_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        selection_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }
}