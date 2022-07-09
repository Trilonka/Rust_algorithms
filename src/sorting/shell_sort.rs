pub fn shell_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    fn insertion<T: PartialOrd + Copy>(arr: &mut [T], start: usize, gap: usize) {
        for i in ((start + gap)..arr.len()).step_by(gap) {
            let val_current = arr[i];
            let mut pos = i;
            while pos >= gap && arr[pos-gap] > val_current {
                arr[pos] = arr[pos-gap];
                //or '' arr.swap(pos, pos-gap); '' instead of 7 and 11 lines
                pos-=gap;
            }
            arr[pos] = val_current;
        }
    }

    let mut count_sublist = arr.len()/2;
    while count_sublist > 0 {
        for pos_start in 0..count_sublist {
            insertion(arr, pos_start, count_sublist);
        }
        count_sublist /= 2;
    }
}

#[cfg(test)]
mod test {
    use super::shell_sort;

    #[test]
    fn basic() {
        let mut vec = vec![3, 5, 6, 3, 1, 4];
        shell_sort(&mut vec);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }

    #[test]
    fn empty() {
        let mut vec: Vec<i32> = vec![];
        shell_sort(&mut vec);
        assert_eq!(vec, vec![]);
    }

    #[test]
    fn reverse() {
        let mut vec = vec![6, 5, 4, 3, 2, 1];
        shell_sort(&mut vec);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }

    #[test]
    fn already_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        shell_sort(&mut vec);
        for i in 0..vec.len() - 1 {
            assert!(vec[i] <= vec[i + 1]);
        }
    }
}