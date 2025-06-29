struct Min {
    // minimum element and index
    min: i32,
    index: usize,
}
fn selection_sort(mut arr: Vec<i32>) -> Vec<i32> {
    // sort a vector in place
    if arr.len() <= 1 {
        return arr;
    }
    let key = min_vec(&arr);
    arr[key.index] = arr[0];
    arr[0] = key.min;
    return [vec![arr[0]], selection_sort(arr[1..].to_vec())].concat();
}

fn min_vec(arr: &Vec<i32>) -> Min {
    // find the minimum element in a vector and its index
    if arr.is_empty() {
        panic!("Cannot find minimum of an empty vector");
    }
    if arr.len() == 1 {
        return Min {
            min: arr[0],
            index: 0,
        };
    }
    let mut min = arr[0];
    let mut min_index = 0;
    for (index, &num) in arr.iter().enumerate() {
        if num < min {
            min = num;
            min_index = index;
        }
    }
    Min {
        min: min,
        index: min_index,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_vector() {
        let arrgh = vec![9, 8, 7, 6, 5, 34, 3, 21, 2];
        let sorted_arrgh = vec![2, 3, 5, 6, 7, 8, 9, 21, 34];
        assert_eq!(selection_sort(arrgh), sorted_arrgh);
    }

    #[test]
    fn test_empty_vector() {
        let empty_vec: Vec<i32> = vec![];
        assert_eq!(selection_sort(empty_vec.clone()), empty_vec);
    }

    #[test]
    fn test_single_element() {
        let single_element_vec = vec![42];
        assert_eq!(selection_sort(single_element_vec.clone()), single_element_vec);
    }

    #[test]
    fn test_sorted_vector() {
        let sorted_vec = vec![1, 2, 3, 4, 5];
        assert_eq!(selection_sort(sorted_vec.clone()), sorted_vec);
    }

    #[test]
    fn test_reverse_sorted_vector() {
        let reverse_sorted_vec = vec![5, 4, 3, 2, 1];
        let sorted_reverse_vec = vec![1, 2, 3, 4, 5];
        assert_eq!(selection_sort(reverse_sorted_vec), sorted_reverse_vec);
    }

    #[test]
    fn test_duplicate_elements() {
        let duplicate_elements_vec = vec![4, 2, 4, 3, 2];
        let sorted_duplicate_elements_vec = vec![2, 2, 3, 4, 4];
        assert_eq!(selection_sort(duplicate_elements_vec), sorted_duplicate_elements_vec);
    }

    #[test]
    fn test_negative_numbers() {
        let negative_numbers_vec = vec![-3, -1, -4, -2, 0];
        let sorted_negative_numbers_vec = vec![-4, -3, -2, -1, 0];
        assert_eq!(selection_sort(negative_numbers_vec), sorted_negative_numbers_vec);
    }

    #[test]
    fn test_mixed_numbers() {
        let mixed_numbers_vec = vec![3, -1, 4, -2, 0];
        let sorted_mixed_numbers_vec = vec![-2, -1, 0, 3, 4];
        assert_eq!(selection_sort(mixed_numbers_vec), sorted_mixed_numbers_vec);
    }
}
