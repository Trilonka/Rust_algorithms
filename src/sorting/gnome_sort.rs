pub fn gnome_sort<T: PartialOrd>(arr: &mut [T]) {
    let mut i = 1;
    let mut j = 2;
    while i < arr.len() {
        if arr[i-1] <= arr[i] {
            i = j;
            j+=1;
        } else {
            arr.swap(i-1, i);
            i-=1;
            if i == 0 {
                i = j;
                j+=1;
            }
        }
    }
}

// not optimized gnome-sort
pub fn gnome_sort_not_optimized<T: PartialOrd>(arr: &mut [T]) {
    let mut gap = 1;
    while gap < arr.len() {
        if arr[gap-1] <= arr[gap] {
            gap+=1;
        } else {
            arr.swap(gap-1, gap);
            gap-=1;
            if gap == 0 {
                gap = 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut arr = vec![5, 2, 1, 3, 4, 6];
        gnome_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn empty() {
        let mut arr = Vec::<i32>::new();
        gnome_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn one_element() {
        let mut arr = vec![1];
        gnome_sort(&mut arr);
        assert_eq!(arr, vec![1]);
    }

    #[test]
    fn pre_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        gnome_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }
}