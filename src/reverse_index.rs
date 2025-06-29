fn index_sort(mut arr: Vec<i32>) -> Vec<i32> {
    // Index sort a vector in place
    for i in 1..arr.len() {
        // select the next element
        let key = arr[i];
        let mut j = i - 1;
        while arr[j] < key {
            arr[j + 1] = arr[j];
            arr[j]=key;
            if j == 0 {break};
            j = j - 1;
        }
    }
    arr
}
fn main() {
    // reverse tests
    let arrgh = vec![9, 8, 7, 6, 5, 34, 3, 21, 2];
    let sorted_arrgh = vec![34, 21, 9, 8, 7, 6, 5, 3, 2];
    assert_eq!(index_sort(arrgh), sorted_arrgh);

    // Test with an empty vector
    let empty_vec: Vec<i32> = vec![];
    assert_eq!(index_sort(empty_vec.clone()), empty_vec);

    // Test with a single element
    let single_element_vec = vec![42];
    assert_eq!(index_sort(single_element_vec.clone()), single_element_vec);

    // Test with already sorted vector
    let sorted_vec = vec![5, 4, 3, 2, 1];
    assert_eq!(index_sort(sorted_vec.clone()), sorted_vec);

    // Test with reverse sorted vector
    let reverse_sorted_vec = vec![5, 4, 3, 2, 1];
    let sorted_reverse_vec = vec![1, 2, 3, 4, 5];
    assert_eq!(index_sort(sorted_reverse_vec), reverse_sorted_vec);

    // Test with duplicate elements
    let duplicate_elements_vec = vec![4, 2, 4, 3, 2];
    let sorted_duplicate_elements_vec = vec![4, 4, 3, 2, 2];
    assert_eq!(index_sort(duplicate_elements_vec), sorted_duplicate_elements_vec);

    // Test with negative numbers
    let negative_numbers_vec = vec![-3, -1, -4, -2, 0];
    let sorted_negative_numbers_vec = vec![0, -1, -2, -3, -4];
    assert_eq!(index_sort(negative_numbers_vec), sorted_negative_numbers_vec);

    // Test with a mix of positive and negative numbers
    let mixed_numbers_vec = vec![3, -1, 4, -2, 0];
    let sorted_mixed_numbers_vec = vec![4, 3, 0, -1, -2];
    assert_eq!(index_sort(mixed_numbers_vec), sorted_mixed_numbers_vec);
    println!("All tests passed successfully")
}
