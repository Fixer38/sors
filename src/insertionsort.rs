use super::Sorter;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord
    {
        // Divide slice into
        // sorted | unsorted

        // starts at 1 because list could already be sorted
        for unsorted in 1..slice.len() {
            // slice[unsorted] is not sorted
            // slice[sorted] and place in sorted location before the unsorted slice
            let mut i = unsorted;
            while i > 0 && slice[i-1] > slice[i] {
                slice.swap(i-1, i);
                // Element shifted, need to go back in array
                i -= 1;
            }

        }

    }
}


#[test]
fn it_works() {
    use super::Sorter;

    let mut unsorted_vec = [18, 2, 1, 4, 9];
    InsertionSort::sort(&mut unsorted_vec);
    assert_eq!(unsorted_vec, [1, 2, 4, 9, 18]);
}
